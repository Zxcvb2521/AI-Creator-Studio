use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, process::{Child, Command}, sync::{Arc, Mutex}, thread, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState { Queued, Running, Completed, Failed, Cancelled }

#[derive(Clone, Serialize)]
pub struct JobSnapshot { pub id: String, pub state: JobState, pub message: String, pub result: Option<Value> }

#[derive(Clone)]
pub struct JobManager {
    jobs: Arc<Mutex<HashMap<String, JobSnapshot>>>,
    processes: Arc<Mutex<HashMap<String, Child>>>,
}

impl JobManager {
    pub fn new() -> Self { Self { jobs: Arc::new(Mutex::new(HashMap::new())), processes: Arc::new(Mutex::new(HashMap::new())) } }

    pub fn submit(&self, python: PathBuf, script: PathBuf, root: PathBuf, output_dir: PathBuf, model_type: String, settings: Value) -> Result<String, String> {
        fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {e}"))?;
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Clock error: {e}"))?.as_millis();
        let id = format!("generation-{stamp}");
        let settings_path = output_dir.parent().unwrap_or(&output_dir).join(format!("{id}.json"));
        fs::write(&settings_path, serde_json::to_vec_pretty(&settings).map_err(|e| format!("Invalid settings: {e}"))?)
            .map_err(|e| format!("Failed to write generation settings: {e}"))?;
        self.jobs.lock().unwrap().insert(id.clone(), JobSnapshot { id: id.clone(), state: JobState::Queued, message: "Queued".into(), result: None });
        let jobs = self.jobs.clone();
        let processes = self.processes.clone();
        let job_id = id.clone();
        thread::spawn(move || {
            if let Some(job) = jobs.lock().unwrap().get_mut(&job_id) { job.state = JobState::Running; job.message = "Generating with WanGP…".into(); }
            let mut cmd = Command::new(&python);
            cmd.arg(script).arg("--root").arg(root).arg("--output-dir").arg(&output_dir).arg("generate").arg("--settings").arg(&settings_path);
            let child = match cmd.spawn() {
                Ok(child) => child,
                Err(e) => {
                    let _ = fs::remove_file(&settings_path);
                    if let Some(job) = jobs.lock().unwrap().get_mut(&job_id) { job.state = JobState::Failed; job.message = format!("Failed to launch WanGP adapter with {}: {e}", python.display()); }
                    return;
                }
            };
            processes.lock().unwrap().insert(job_id.clone(), child);
            loop {
                let finished = {
                    let mut table = processes.lock().unwrap();
                    if let Some(child) = table.get_mut(&job_id) { match child.try_wait() { Ok(Some(status)) => Some(status), Ok(None) => None, Err(_) => None } } else { None }
                };
                if let Some(status) = finished {
                    let output = { let mut table = processes.lock().unwrap(); let child = table.remove(&job_id); child.and_then(|mut c| c.wait_with_output().ok()) };
                    let _ = fs::remove_file(&settings_path);
                    let mut table = jobs.lock().unwrap();
                    if let Some(job) = table.get_mut(&job_id) {
                        if status.success() {
                            let stdout = output.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
                            match serde_json::from_str::<Value>(&stdout) { Ok(value) => { job.state = JobState::Completed; job.message = "Generation completed".into(); job.result = Some(value); }, Err(e) => { job.state = JobState::Failed; job.message = format!("Invalid adapter response: {e}"); } }
                        } else {
                            let stderr = output.as_ref().map(|o| String::from_utf8_lossy(&o.stderr).trim().to_string()).unwrap_or_default();
                            job.state = JobState::Failed; job.message = if stderr.is_empty() { "WanGP generation failed".into() } else { stderr };
                        }
                    }
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(250));
            }
        });
        let _ = model_type;
        Ok(id)
    }

    pub fn get(&self, id: &str) -> Result<JobSnapshot, String> { self.jobs.lock().unwrap().get(id).cloned().ok_or_else(|| format!("Unknown generation job: {id}")) }

    pub fn cancel(&self, id: &str) -> Result<JobSnapshot, String> {
        let state = self.get(id)?;
        if matches!(state.state, JobState::Completed | JobState::Failed | JobState::Cancelled) { return Ok(state); }
        if let Some(mut child) = self.processes.lock().unwrap().remove(id) { child.kill().map_err(|e| format!("Failed to cancel generation: {e}"))?; let _ = child.wait(); }
        let mut jobs = self.jobs.lock().unwrap();
        let job = jobs.get_mut(id).ok_or_else(|| format!("Unknown generation job: {id}"))?;
        job.state = JobState::Cancelled; job.message = "Generation cancelled".into(); job.result = None;
        Ok(job.clone())
    }
}
