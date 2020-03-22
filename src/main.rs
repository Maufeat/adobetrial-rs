use rand::Rng;
use regex::Regex;
use std::{
    fs,
    io::{stdin, stdout, Write},
    ops::{Bound, RangeBounds},
};
use str_replace::StrReplace;

mod str_replace;

struct AdobeProduct {
    path: String,
    folder_name: String,
    application_files: Vec<String>,
}

impl AdobeProduct {
    fn new(path: &str, name: &str) -> AdobeProduct {
        AdobeProduct {
            path: path.to_string(),
            folder_name: name.to_string(),
            application_files: vec![],
        }
    }

    fn merge_application_files(&mut self, new_application_files: Vec<String>) {
        self.application_files
            .extend(new_application_files.iter().cloned());
    }

    fn patch_trial(&mut self) {
        for application_file in &self.application_files {
            let mut reader = StrReplace::from_file(application_file);
            let rex = Regex::new("(<Data key=\"TrialSerialNumber\">)(.*)(</Data>)").unwrap();
            let mut key_regex = vec![];

            for result in rex.captures_iter(reader.to_str()) {
                key_regex.push(result[1].to_string());
                key_regex.push(result[2].to_string());
                key_regex.push(result[3].to_string());
            }

            let original_key_slice = key_regex[1].substring(0, 15).to_string();

            reader.replace(
                format!(
                    "{opening}{key}{closing}",
                    opening = key_regex[0],
                    key = key_regex[1],
                    closing = key_regex[2]
                )
                .as_str(),
                format!(
                    "{opening}{key_slice}{rnd_key}{closing}",
                    opening = key_regex[0],
                    key_slice = original_key_slice,
                    rnd_key = gen_key_range(9),
                    closing = key_regex[2]
                )
                .as_str(),
            );

            match reader.to_file(application_file) {
                Ok(_) => println!("Successfully Patched {}", self.folder_name),
                Err(e) => println!(
                    "Couldn't Patch {} | Reason: {:?}",
                    self.folder_name,
                    e.to_string()
                ),
            }
        }
    }
}

fn main() {
    let mut dir = String::new();
    println!("Enter your Adobe Directory: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut dir)
        .expect("Did not enter a correct string");

    dir = dir.replace('\r', "").replace('\n', "");

    let mut adobe_products = vec![];

    match fs::read_dir(dir) {
        Ok(dir) => {
            for path in dir {
                match path {
                    Ok(entry) => {
                        if entry.path().is_dir()
                            && String::from(entry.path().file_name().unwrap().to_str().unwrap())
                                .contains("Adobe")
                        {
                            let mut product = AdobeProduct::new(
                                &entry.path().display().to_string(),
                                &entry
                                    .path()
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                            );
                            product.merge_application_files(get_application_files(&product.path));
                            adobe_products.push(product);
                        }
                    }
                    Err(_error) => (),
                };
            }
        }
        Err(reason) => {
            println!("Unable ro read directory | Reason: {}", reason.to_string());
        }
    };

    if adobe_products.is_empty() {
        println!("Found no Adobe Products");
    } else {
        println!("Select the Adobe Product where you want to reset your trial:");
        for (i, product) in adobe_products.iter().enumerate() {
            println!("{}) {}", i + 1, product.folder_name);
        }
        println!("{}) Reset All", adobe_products.len() + 1);

        let mut choice_str = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut choice_str).expect("No valid input");

        let choice_nbr: usize = choice_str.trim().parse().unwrap();

        if choice_nbr == adobe_products.len() + 1 {
            for mut adobe_product in adobe_products {
                adobe_product.patch_trial();
            }
        } else {
            adobe_products[choice_nbr - 1].patch_trial();
        }
    }
}

fn gen_key_range(len: u16) -> String {
    let mut key = String::from("");
    let mut rng = rand::thread_rng();
    for _nbr in 0..len {
        key.push_str(&rng.gen_range(0, 9).to_string());
    }
    key
}

fn get_application_files(dir: &str) -> Vec<String> {
    let mut application_files: Vec<String> = vec![];

    match fs::read_dir(dir) {
        Ok(dir) => {
            for path in dir {
                match path {
                    Ok(entry) => {
                        if entry.path().is_dir() {
                            application_files.extend(
                                get_application_files(&entry.path().display().to_string())
                                    .iter()
                                    .cloned(),
                            );
                        } else if entry.path().file_name().unwrap().to_str().unwrap()
                            == "application.xml"
                        {
                            application_files.push(entry.path().display().to_string());
                        }
                    }
                    Err(_error) => (),
                };
            }
        }
        Err(reason) => {
            println!("Unable ro read directory | Reason: {}", reason.to_string());
        }
    }

    application_files
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}
