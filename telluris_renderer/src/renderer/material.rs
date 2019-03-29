use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Material {
    id: usize,
    render_queue: usize,
    pipeline_id: usize,
}

#[derive(Debug)]
struct Pipeline {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct PipelineManager {
    pipelines: HashMap<usize, Arc<Pipeline>>,
}
