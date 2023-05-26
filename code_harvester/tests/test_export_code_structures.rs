#[cfg(test)]
mod tests {
    use code_harvester::export_code_structures::get_module_name;
    use code_harvester::{export_code_structures, extract_code_structures};
    use std::path::Path;

    #[test]
    fn test_integration_export_code_structures() -> std::io::Result<()> {
        // Read and parse the sample Python file
        let sample_file_path = "code_harvester/samples/sample.py";
        let file_content = std::fs::read_to_string(sample_file_path)?;
        let code_structures = extract_code_structures(&file_content);

        // Get the module name and create a temporary directory for the exported files
        let module_name = get_module_name(sample_file_path)?;
        let destination_folder = "code_harvester/tests/integration_exported_structures";
        std::fs::create_dir_all(destination_folder)?;

        // Export the code structures
        export_code_structures(&code_structures, &module_name, destination_folder)?;

        // Verify the exported files
        let foo_path = Path::new(destination_folder).join("sample_foo_function.txt");
        let bar_path = Path::new(destination_folder).join("sample_Bar_class.txt");

        assert!(foo_path.exists());
        assert!(bar_path.exists());

        let foo_content = std::fs::read_to_string(&foo_path)?;
        let bar_content = std::fs::read_to_string(&bar_path)?;

        assert_eq!(foo_content, "def foo():\n    return 42");
        assert_eq!(bar_content, "class Bar:\n    def __init__(self, x):\n        self.x = x\n\n    def get_x(self):\n        return self.x");

        // Clean up the exported files
        std::fs::remove_file(foo_path)?;
        std::fs::remove_file(bar_path)?;
        std::fs::remove_dir(destination_folder)?;

        Ok(())
    }
}
