#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn add_workflow_test_pass() {
        let composer = Composer::default();

        let workflow1 = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks: HashMap::default(),
        };

        composer
            .add_workflow(
                "test-workflow".to_string(),
                "0.0.1".to_string(),
                HashMap::default(),
            )
            .unwrap();

        let composer_workflow = &composer.workflows.borrow()[0];

        assert_eq!(composer_workflow, &workflow1);
    }

    #[test]
    fn get_dependencies_test() {
        let composer = Composer::default();

        let mut dependencies: Vec<Depend> = Vec::new();
        dependencies.push(Depend {
            task_name: "dependent_task".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });

        let task = Task {
            action_name: "get_salaries".to_string(),
            depend_on: dependencies,
            ..Default::default()
        };

        let mut tasks = HashMap::<String, Task>::new();
        tasks.insert("get_salaries".to_string(), task);

        composer
            .add_workflow("test-workflow".to_string(), "0.0.1".to_string(), tasks)
            .unwrap();

        assert_eq!(
            composer.workflows.borrow()[0].get_dependencies("get_salaries").unwrap(),
            vec!["dependent_task"]
        );
    }

    #[test]
    fn get_flow_test() {
        let composer = Composer::default();

        let task0 = Task {
            action_name: "task0".to_string(),
            ..Default::default()
        };
        let mut task1 = Task {
            action_name: "task1".to_string(),
            ..Default::default()
        };

        let mut dependencies: Vec<Depend> = Vec::new();
        dependencies.push(Depend {
            task_name: "task0".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        dependencies.push(Depend {
            task_name: "task4".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        task1.depend_on = dependencies;

        let mut task2 = Task {
            action_name: "task2".to_string(),
            ..Default::default()
        };

        let mut dependencies: Vec<Depend> = Vec::new();
        dependencies.push(Depend {
            task_name: "task0".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        task2.depend_on = dependencies;

        let mut task3 = Task {
            action_name: "task3".to_string(),
            ..Default::default()
        };

        let mut dependencies: Vec<Depend> = Vec::new();
        dependencies.push(Depend {
            task_name: "task1".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        dependencies.push(Depend {
            task_name: "task2".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        task3.depend_on = dependencies;

        let task4 = Task {
            action_name: "task4".to_string(),
            ..Default::default()
        };
        let mut task5 = Task {
            action_name: "task5".to_string(),
            ..Default::default()
        };

        let mut dependencies: Vec<Depend> = Vec::new();
        dependencies.push(Depend {
            task_name: "task2".to_string(),
            cur_field: "id".to_string(),
            prev_field: "ids".to_string(),
        });
        task5.depend_on = dependencies;

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);
        tasks.insert("task1".to_string(), task1);
        tasks.insert("task2".to_string(), task2);
        tasks.insert("task3".to_string(), task3);
        tasks.insert("task4".to_string(), task4);
        tasks.insert("task5".to_string(), task5);

        composer
            .add_workflow("test-workflow".to_string(), "0.0.1".to_string(), tasks)
            .unwrap();

        let flow = composer.workflows.borrow()[0].get_flow();

        assert!(flow[0] == "task0" || flow[0] == "task4");

        assert!(flow[1] == "task0" || flow[1] == "task4" || flow[1] == "task2");

        assert!(
            flow[2] == "task1" || flow[2] == "task2" || flow[2] == "task4" || flow[2] == "task5"
        );

        assert!(
            flow[3] == "task1" || flow[3] == "task2" || flow[3] == "task4" || flow[3] == "task5"
        );

        assert!(flow[4] == "task3" || flow[4] == "task5" || flow[4] == "task1");

        assert!(flow[5] == "task3" || flow[5] == "task5");
    }

    #[test]
    fn get_attributes_test() {
        let composer = Composer::default();

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("namespace".to_string(), "value1".to_string());
        attributes.insert("auth_key".to_string(), "value2".to_string());

        let mut tasks = HashMap::new();
        tasks.insert(
            "test-task".to_string(),
            Task {
                attributes,
                ..Default::default()
            },
        );

        composer
            .add_workflow("test-workflow".to_string(), "0.0.1".to_string(), tasks)
            .unwrap();

        let composer_task = &composer.workflows.borrow()[0].tasks;

        let attributes =
            parse_module::get_attributes(&composer_task.get("test-task").unwrap().attributes);

        println!("{:#?}", attributes);

        assert!(
            attributes == "[Namespace:\"value1\",AuthKey:\"value2\"]"
                || attributes == "[AuthKey:\"value2\",Namespace:\"value1\"]"
        );
    }

    #[test]
    fn get_task_kind_test_pass() {
        let kind_name = get_task_kind("polkadot").unwrap();
        assert_eq!(&kind_name, "Polkadot");

        let kind_name = get_task_kind("openwhisk").unwrap();
        assert_eq!(&kind_name, "OpenWhisk");
    }

    #[test]
    #[should_panic]
    fn get_task_kind_test_fail() {
        let kind_name = get_task_kind("polkadot").unwrap();
        assert_eq!(&kind_name, "polkadot");
    }
}
