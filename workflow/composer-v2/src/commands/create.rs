use clap::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Create {
    pub package_name: String,
}

impl Create {
    pub fn execute(self) {
        let current = std::env::current_dir().unwrap();
        let package = current.join(self.package_name.clone());
        std::fs::create_dir_all(package.clone()).unwrap();
        let temp_path = package.as_path().join("main.echo");

        let content = format!(
            "workflows(
            name = {},
            version = \"0.0.1\",
            tasks = []
        )",
            self.package_name
        );
        std::fs::write(temp_path, content.as_bytes()).unwrap();
    }
}
