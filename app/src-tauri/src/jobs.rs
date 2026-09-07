use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState { Queued, Running, Completed, Failed, Cancelled }

#[derive(Clone, serde::Serialize)]
pub struct JobSnapshot { pub id: String, pub state: JobState, pub message: String, pub result: Option<Value> }

pub struct JobManager {
    jobs: Arc<Mutex<std::collections::HashMap<String, JobSnapshot>>>,
    processes: Arc<Mutex<std::collections::HashMap<String, Child>>>,
}

impl JobManager {
    pub fn new() -> Self { Self { jobs: Arc::new(Mutex::new(std::collections::HashMap::new())), processes: Arc::new(Mutex::new(std::collections::HashMap::new())) } }

    pub fn submit(&self, python: PathBuf, script: PathBuf, root: PathBuf, output_dir: PathBuf, model_type: String, settings: Value) -> Result<String,String> {
        let id = Uuid::new_v4().to_string();
        fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {e}"))?;
        let settings_path = output_dir.join(format!("{id}.json"));
        fs::write(&settings_path, serde_json::to_vec_pretty(&settings).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let stdout_path = output_dir.join(format!("{id}.stdout.log"));
        let stderr_path = output_dir.join(format!("{id}.stderr.log"));
        self.jobs.lock().unwrap().insert(id.clone(), JobSnapshot { id: id.clone(), state: JobState::Queued, message: "Queued…".into(), result: None });
        let jobs = self.jobs.clone(); let processes = self.processes.clone(); let job_id = id.clone();
        thread::spawn(move || {
            if let Some(job) = jobs.lock().unwrap().get_mut(&job_id) { job.state = JobState::Running; job.message = "Running LTX…".into(); }
            let stdout = match fs::File::create(&stdout_path) { Ok(v) => v, Err(e) => { fail_job(&jobs,&job_id,format!("Failed to create stdout log: {e}")); return; } };
            let stderr = match fs::File::create(&stderr_path) { Ok(v) => v, Err(e) => { fail_job(&jobs,&job_id,format!("Failed to create stderr log: {e}")); return; } };
            let mut command = Command::new(&python);
            command.current_dir(&root).env("PYTHONNOUSERSITE","1").env("PYTHONUTF8","1").env("PYTHONUNBUFFERED","1").env("WAN2GP_ROOT",&root)
                .arg(&script).arg("--root").arg(&root).arg("generate").arg("--model").arg(&model_type).arg("--settings").arg(&settings_path)
                .stdout(Stdio::from(stdout)).stderr(Stdio::from(stderr));
            let child = match command.spawn() { Ok(v) => v, Err(e) => { fail_job(&jobs,&job_id,format!("Failed to launch LTX adapter with {}: {e}",python.display())); cleanup(&settings_path,&stdout_path,&stderr_path); return; } };
            processes.lock().unwrap().insert(job_id.clone(), child);
            loop {
                let finished = { let mut table = processes.lock().unwrap(); match table.get_mut(&job_id) { Some(child) => match child.try_wait() { Ok(Some(status)) => Some(status), Ok(None) => None, Err(e) => { fail_job(&jobs,&job_id,format!("Failed while polling LTX process: {e}")); None } }, None => None } };
                if let Some(status) = finished {
                    let child = processes.lock().unwrap().remove(&job_id);
                    let _ = child.and_then(|mut c| c.wait().ok());
                    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
                    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
                    let debug_path = output_dir.join(format!("{job_id}.log"));
                    let _ = fs::write(&debug_path, format!("=== LTX job {job_id} ===\nexit_code: {:?}\nsuccess: {}\n\n=== STDOUT ===\n{}\n\n=== STDERR ===\n{}\n",status.code(),status.success(),stdout.trim(),stderr.trim()));
                    let mut table = jobs.lock().unwrap();
                    if let Some(job) = table.get_mut(&job_id) {
                        if status.success() { match parse_adapter_json(&stdout) { Ok(value) => { job.state=JobState::Completed; job.message="Generation completed".into(); job.result=Some(value); }, Err(e) => { job.state=JobState::Failed; job.message=format!("Invalid adapter response: {e}. Raw output saved to {}",debug_path.display()); } } }
                        else { job.state=JobState::Failed; job.message=if !stderr.trim().is_empty(){stderr.trim().into()}else if !stdout.trim().is_empty(){stdout.trim().into()}else{"LTX generation failed".into()}; }
                    }
                    cleanup(&settings_path,&stdout_path,&stderr_path); break;
                }
                thread::sleep(std::time::Duration::from_millis(250));
            }
        }); Ok(id)
    }

    pub fn get(&self,id:&str)->Result<JobSnapshot,String>{ self.jobs.lock().unwrap().get(id).cloned().ok_or_else(||"Job not found".into()) }

    pub fn cancel(&self,id:&str)->Result<JobSnapshot,String>{
        if !self.jobs.lock().unwrap().contains_key(id) { return Err("Job not found".into()); }
        if let Some(mut child)=self.processes.lock().unwrap().remove(id) { let _=child.kill(); let _=child.wait(); }
        if let Some(job)=self.jobs.lock().unwrap().get_mut(id) { if matches!(job.state,JobState::Queued|JobState::Running){job.state=JobState::Cancelled;job.message="Generation cancelled".into();} return Ok(job.clone()); }
        Err("Job not found".into())
    }
}

fn fail_job(jobs:&Arc<Mutex<std::collections::HashMap<String,JobSnapshot>>>,id:&str,message:String){ if let Some(job)=jobs.lock().unwrap().get_mut(id){job.state=JobState::Failed;job.message=message;} }
fn cleanup(settings:&PathBuf,stdout:&PathBuf,stderr:&PathBuf){let _=fs::remove_file(settings);let _=fs::remove_file(stdout);let _=fs::remove_file(stderr);}
fn parse_adapter_json(stdout:&str)->Result<Value,String>{let trimmed=stdout.trim();if trimmed.is_empty(){return Err("Adapter returned empty stdout".into());}if let Ok(v)=serde_json::from_str::<Value>(trimmed){return Ok(v);}for line in trimmed.lines().rev(){if let Ok(v)=serde_json::from_str::<Value>(line.trim()){return Ok(v);}}Err(format!("Adapter stdout is not valid JSON. First output: {}",stdout.lines().next().unwrap_or("<empty>")))}
