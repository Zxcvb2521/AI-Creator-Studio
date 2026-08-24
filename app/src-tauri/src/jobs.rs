use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState { Queued, Running, Completed, Failed, Cancelled }
#[derive(Clone, serde::Serialize)]
pub struct JobSnapshot { pub id: String, pub state: JobState, pub message: String, pub result: Option<Value> }
pub struct JobManager { jobs: Arc<Mutex<std::collections::HashMap<String, JobSnapshot>>>, processes: Arc<Mutex<std::collections::HashMap<String, Child>>> }
impl JobManager {
 pub fn new() -> Self { Self { jobs: Arc::new(Mutex::new(std::collections::HashMap::new())), processes: Arc::new(Mutex::new(std::collections::HashMap::new())) } }
 pub fn submit(&self, python: PathBuf, script: PathBuf, root: PathBuf, output_dir: PathBuf, model_type: String, settings: Value) -> Result<String,String> {
  let id=Uuid::new_v4().to_string(); fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {e}"))?;
  let settings_path=output_dir.join(format!("{id}.json")); fs::write(&settings_path, serde_json::to_vec_pretty(&settings).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
  self.jobs.lock().unwrap().insert(id.clone(), JobSnapshot{id:id.clone(),state:JobState::Queued,message:"Queued…".into(),result:None});
  let jobs=self.jobs.clone(); let processes=self.processes.clone(); let job_id=id.clone();
  thread::spawn(move || {
   if let Some(job)=jobs.lock().unwrap().get_mut(&job_id){job.state=JobState::Running;job.message="Running WanGP…".into();}
   let mut command=Command::new(&python);
   command.current_dir(&root)
       .env("PYTHONNOUSERSITE", "1")
       .env("PYTHONUTF8", "1")
       .env("PYTHONUNBUFFERED", "1")
       .env("WAN2GP_ROOT", &root)
       .arg(&script).arg("--root").arg(&root).arg("generate").arg("--model").arg(&model_type).arg("--settings").arg(&settings_path);
   let child=match command.spawn(){Ok(c)=>c,Err(e)=>{if let Some(job)=jobs.lock().unwrap().get_mut(&job_id){job.state=JobState::Failed;job.message=format!("Failed to launch Wan2GP adapter with {}: {e}",python.display());}return;}};
   processes.lock().unwrap().insert(job_id.clone(),child);
   loop {
    let finished={let mut table=processes.lock().unwrap();if let Some(child)=table.get_mut(&job_id){match child.try_wait(){Ok(Some(status))=>Some(status),Ok(None)=>None,Err(_)=>None}}else{None}};
    if let Some(status)=finished {
     let output={let mut table=processes.lock().unwrap();let child=table.remove(&job_id);child.and_then(|c|c.wait_with_output().ok())};
     let _=fs::remove_file(&settings_path);
     let mut table=jobs.lock().unwrap(); if let Some(job)=table.get_mut(&job_id){
      if status.success(){let stdout=output.as_ref().map(|o|String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();match serde_json::from_str::<Value>(&stdout){Ok(value)=>{job.state=JobState::Completed;job.message="Generation completed".into();job.result=Some(value)},Err(e)=>{job.state=JobState::Failed;job.message=format!("Invalid adapter response: {e}")}}}else{let stderr=output.as_ref().map(|o|String::from_utf8_lossy(&o.stderr).trim().to_string()).unwrap_or_default();job.state=JobState::Failed;job.message=if stderr.is_empty(){"WanGP generation failed".into()}else{stderr};}
     } break;
    }
    thread::sleep(std::time::Duration::from_millis(250));
   }
  }); Ok(id)
 }
 pub fn get(&self,id:&str)->Result<JobSnapshot,String>{self.jobs.lock().unwrap().get(id).cloned().ok_or_else(||"Job not found".into())}
 pub fn cancel(&self,id:&str)->Result<JobSnapshot,String>{if let Some(mut child)=self.processes.lock().unwrap().remove(id){let _=child.kill();}if let Some(job)=self.jobs.lock().unwrap().get_mut(id){job.state=JobState::Cancelled;job.message="Generation cancelled".into();return Ok(job.clone())}Err("Job not found".into())}
}
