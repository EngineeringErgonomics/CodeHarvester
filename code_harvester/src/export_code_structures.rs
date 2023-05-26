use crate::code_structures::CodeStructure;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_code_structures(
    structures: &[CodeStructure],
    module_name: &str,
    destination_folder: &str,
) -> std::io::Result<()> {
    for structure in structures {
        let file_name = format!(
            "{}_{}_{}.txt",
            module_name,
            structure.get_name(),
            structure.get_type()
        );
        let file_path = Path::new(destination_folder).join(file_name);
        println!("File path: {:?}", file_path);

        let mut file = File::create(&file_path)?;
        file.write_all(structure.get_content().as_bytes())?;
    }

    Ok(())
}

pub fn get_module_name(file_path: &str) -> std::io::Result<String> {
    let path = Path::new(file_path);
    let file_stem = path.file_stem().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file path")
    })?;
    let module_name = file_stem
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name"))?;
    Ok(module_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module_name() -> std::io::Result<()> {
        let file_path = "code_harvester/samples/sample.py";
        let expected_module_name = "sample";

        let module_name = get_module_name(file_path)?;
        assert_eq!(module_name, expected_module_name);

        Ok(())
    }

    #[test]
    fn test_export_code_structures() -> std::io::Result<()> {
        let structures = vec![
        CodeStructure::Function("def foo():\n    return 42".to_string()),
        CodeStructure::Class("class Bar:\n    def __init__(self, x):\n        self.x = x\n\n    def get_x(self):\n        return self.x".to_string()),
    ];

        let destination_folder = "code_harvester/tests/exported_structures";
        let module_name = "sample";
        std::fs::create_dir_all(destination_folder)?;

        // Print the names and types of the CodeStructure instances
        for structure in &structures {
            println!(
                "Name: {}, Type: {}",
                structure.get_name(),
                structure.get_type()
            );
        }

        export_code_structures(&structures, module_name, destination_folder)?;

        let foo_path = Path::new(destination_folder).join("sample_foo_function.txt");
        let bar_path = Path::new(destination_folder).join("sample_Bar_class.txt");

        println!("Foo path: {:?}", foo_path);
        println!("Bar path: {:?}", bar_path);

        if foo_path.exists() {
            let foo_content = std::fs::read_to_string(&foo_path)?;
            println!("Foo content: {:?}", foo_content);
        }

        if bar_path.exists() {
            let bar_content = std::fs::read_to_string(&bar_path)?;
            println!("Bar content: {:?}", bar_content);
        }

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
