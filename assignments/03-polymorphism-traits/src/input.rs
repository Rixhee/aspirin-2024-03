use anyhow::Result;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

trait GetInput {
    fn get_lines(&self) -> Result<Box<dyn Iterator<Item = String>>>;
}

struct StringInput {
    string: String,
}

struct FilePathInput {
    path: PathBuf,
}

impl GetInput for StringInput {
    fn get_lines(&self) -> Result<Box<dyn Iterator<Item = String>>> {
        Ok(Box::new(
            self.string
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .into_iter(),
        ))
    }
}

impl GetInput for FilePathInput {
    fn get_lines(&self) -> Result<Box<dyn Iterator<Item = String>>> {
        let file = std::fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|l| l.unwrap());

        Ok(Box::new(lines))
    }
}

pub fn get_lines_from_input(file: Option<PathBuf>) -> Result<Box<dyn Iterator<Item = String>>> {
    if let Some(path) = file {
        Ok(Box::new(FilePathInput { path }.get_lines()?))
    } else {
        let stdin = io::stdin();
        let string_input = stdin.lock().lines().collect::<Result<String, _>>()?;
        Ok(Box::new(
            StringInput {
                string: string_input,
            }
            .get_lines()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // Helper function to create a temporary file for testing
    fn create_temp_file(content: &str) -> PathBuf {
        let path = std::env::temp_dir().join("test_file.txt");
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path
    }

    #[test]
    fn test_string_input() {
        let input_string = "Line 1\nLine 2\nLine 3\n".to_string();
        let string_input = StringInput {
            string: input_string,
        };
        let lines = string_input.get_lines().unwrap();
        let collected_lines: Vec<String> = lines.collect();

        assert_eq!(collected_lines, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_get_lines_from_input_with_file() {
        let temp_file_path = create_temp_file("File input line 1\nFile input line 2\n");
        let result = get_lines_from_input(Some(temp_file_path)).unwrap();
        let collected_lines: Vec<String> = result.collect();

        assert_eq!(
            collected_lines,
            vec!["File input line 1", "File input line 2"]
        );
    }
}
