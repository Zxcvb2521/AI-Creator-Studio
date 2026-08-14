use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, process::Command, sync::{Arc, Mutex}, thread, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState { Queued, Running, Completed, Failed, Cancelled }

#[derive(Clone, Serialize)]
pub struct JobSnapshot {
    pub id: String,
    pub state: JobState,
    pub message: String,
    pub result: Option<Value>,
}

#[derive(Clone)]
pub struct JobManager { jobs: Arc<Mutex<HashMap<String, JobSnapshot>>> }

impl JobManager {
    pub fn new() -> Self { Self { jobs: Arc::new(Mutex::new(HashMap::new())) } }

    pub fn submit(&self, script: PathBuf, root: PathBuf, output_dir: PathBuf, model_type: String, settings: Value) -> Result<String, String> {
        fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {e}"))?;
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Clock error: {e}"))?.as_millis();
        let id = format!("generation-{stamp}");
        let settings_path = output_dir.parent().unwrap_or(&output_dir).join(format!("{id}.json"));
        fs::write(&settings_path, serde_json::to_vec_pretty(&settings).map_err(|e| format!("Invalid settings: {e}"))?)
            .map_err(|e| format!("Failed to write generation settings: {e}"))?;
        let snapshot = JobSnapshot { id: id.clone(), state: JobState::Queued, message: "Queued".into(), result: None };
        self.jobs.lock().unwrap().insert(id.clone(), snapshot);
        let jobs = self.jobs.clone();
        let job_id = id.clone();
        thread::spawn(move || {
            if let Some(job) = jobs.lock().unwrap().get_mut(&job_id) { job.state = JobState::Running; job.message = "Generating with WanGP…".into(); }
            let mut cmd = if cfg!(target_os = "windows") { Command::new("python") } else { Command::new("python3") };
            cmd.arg(script).arg("--root").arg(root).arg("--output-dir").arg(&output_dir).arg("generate").arg("--settings").arg(&settings_path);
            let result = cmd.output();
            let _ = fs::remove_file(&settings_path);
            let mut table = jobs.lock().unwrap();
            if let Some(job) = table.get_mut(&job_id) {
                match result {
                    Ok(output) if output.status.success() => {
                        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        match serde_json::from_str::<Value>(&stdout) {
                            Ok(value) => { job.state = JobState::Completed; job.message = "Generation completed".into(); job.result = Some(value); }
                            Err(e) => { job.state = JobState::Failed; job.message = format!("Invalid adapter response: {e}"); }
                        }
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                        job.state = JobState::Failed;
                        job.message = if stderr.is_empty() { "WanGP generation failed".into() } else { stderr };
                    }
                    Err(e) => { job.state = JobState::Failed; job.message = format!("Failed to launch WanGP adapter: {e}"); }
                }
            }
        });
        Ok(id)
    }

    pub fn get(&self, id: &str) -> Result<JobSnapshot, String> {
        self.jobs.lock().unwrap().get(id).cloned().ok_or_else(|| format!("Unknown generation job: {id}"))
    }
}
