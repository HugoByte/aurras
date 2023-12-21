use super::*;

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub name: String,
    pub version: String,
    pub tasks: HashMap<String, Task>,
}

impl Workflow{
    /// Finds the list of dependencies that the given task depends on.
    ///
    /// # Arguments
    ///
    /// * `task_name` - A string slice that holds the name of the task
    /// * `workflow_index` - A integer that holds the index of the workflow where the given
    ///   task is stored
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - An option containing a vector of dependencies if the task is
    ///   found, or None if the task have no dependency
    ///
    pub fn get_dependencies(&self, task_name: &str) -> Option<Vec<String>> {
        let mut dependencies = Vec::<String>::new();

        for task in self.tasks.get(task_name).unwrap().depend_on.iter() {
            dependencies.push(task.task_name.clone());
        }

        Some(dependencies)
    }

    /// Performs depth-first search (DFS) in the workflow subgraph.
    /// This method is invoked within the get_flow method to perform `Topological-Sorting`
    /// # Arguments
    ///
    /// * `task_name` - A string slice that holds the name of the task where the DFS should start
    /// * `visited` - A mutable reference to a HashMap that holds the list of task (node) names
    ///   and a boolean indicating whether it has been traversed
    /// * `flow` - A mutable reference to a vector of strings that stores the flow of the DFS
    ///   traversal
    /// * `workflow_index` - An integer that holds the index of the workflow where the given
    ///   task is located
    ///
    fn dfs(
        &self,
        task_name: &str,
        visited: &mut HashMap<String, bool>,
        flow: &mut Vec<String>,
    ) {
        visited.insert(task_name.to_string(), true);

        for depend_task in self.get_dependencies(task_name).unwrap().iter() {
            if !visited[depend_task] {
                self.dfs(depend_task, visited, flow);
            }
        }

        flow.push(task_name.to_string());
    }

    /// Performs topological sort in the workflow graph.
    /// This method is invoked by the parse_module.
    ///
    /// # Arguments
    ///
    /// * `workflow_index` - An integer that holds the index of the workflow for which
    ///   topological sort is to be performed
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - A vector containing the list of task names in the order of the
    ///   topological sort
    ///
    pub fn get_flow(&self) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for task in self.tasks.iter() {
            visited.insert(task.0.to_string(), false);
        }

        for task in self.tasks.iter() {
            if !visited[task.0] {
                self.dfs(task.0, &mut visited, &mut flow)
            }
        }

        flow
    }
}