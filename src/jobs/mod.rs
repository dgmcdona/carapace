use std::fmt;
use std::process::Child;
use crate::command::CommandType;

pub enum JobType {
    Foreground,
    Background,
}

#[derive(Debug)]
pub enum State {
    Running,
    Stopped,
    Terminated,
}

pub struct Job {
    command: CommandType,
    jobtype: JobType,
    child_handle: Child,
    state: State,
}

impl Job {
    pub fn new(command: CommandType, jobtype: JobType, child_handle: Child) -> Job {
        Job {
            command,
            jobtype,
            child_handle,
            state: State::Running,
        }
    }

    pub fn get_command_type(&self) -> &CommandType {
        &self.command
    }

    pub fn get_job_type(&self) -> &JobType {
        &self.jobtype
    }

    pub fn get_child_handle(&self) -> &Child {
        &self.child_handle
    } 
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.command {
            CommandType::Foreground(argv) => {
                write!(f, "{:?}: {}", self.state, argv.join(" "))
            }
            CommandType::Background(argv) => {
                write!(f, "{:?}: {}", self.state, argv.join(" "))
            }
            CommandType::Builtin(argv) => {
                write!(f, "{:?}: {}", self.state, argv.join(" "))
            }
        }
    }
}

pub struct JobList {
    jobs: Vec<Option<(usize, Job)>>,
    fg_id: Option<usize>,
    count: u32,
}

impl JobList {
    pub fn new() -> JobList {
        JobList {
            jobs: Vec::new(),
            fg_id: None,
            count: 0
        }
    }
    pub fn add_fg(&mut self, job: Job) {
        self.fg_id = Some(self.add_job(job));
    }

    fn add_job(&mut self, job: Job) -> usize {
        self.count += 1;
        let mut found_slot: Option<usize> = None;
        for (i, job_slot) in self.jobs.iter_mut().enumerate() {
            match job_slot {
                Some(_) => {}
                None => {
                    found_slot = Some(i);
                    break;
                }
            };
        }
        match found_slot {
            Some(slot_index) => {
                self.jobs[slot_index] = Some((slot_index, job));
                slot_index
            }
            None => {
                self.jobs.push(Some((self.jobs.len(), job)));
                self.jobs.len()
            }
        }
    }

    pub fn add_bg(&mut self, job: Job) {
        self.add_job(job); 
    }

    pub fn list_jobs(&self) {
        for job in &self.jobs {
            match job {
                Some((i, job)) => {
                    println!("[{}] {}", i, job)
                }
                None => {}
            }
        }
    }
}
