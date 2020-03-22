use std::{fs, io::{stdin, stdout, Write}, ops::{Bound, RangeBounds}};
use str_replace::StrReplace;
use regex::Regex;
use rand::Rng;

mod str_replace;
mod windows;

struct AdobeProduct {
    path: String,
    folder_name: String,
    application_files: Vec<String>
}

impl AdobeProduct{ 
  fn new(path: &str, name: &str) -> AdobeProduct {
    AdobeProduct {
        path: path.to_string(), 
        folder_name: name.to_string(),
        application_files: vec![],
    }
  }

  fn merge_application_files(&mut self, new_application_files: Vec<String>){
        self.application_files.extend(new_application_files.iter().cloned());
  }

  fn patch_trial(&mut self){
    if(self.application_files.len() > 0){
        for application_file in &self.application_files {
            let mut reader = StrReplace::from_file(application_file);
            let rex = Regex::new("(<Data key=\"TrialSerialNumber\">)(.*)(</Data>)").unwrap();
            let mut original_key_slice = String::from("");
            let mut key_regex = vec![];
                        
            for cap in rex.captures_iter(reader.to_str()) {
                //println!("{} {} {}", &cap[1], &cap[2], &cap[3]);
                key_regex.push(cap[1].to_string());
                key_regex.push(cap[2].to_string());
                key_regex.push(cap[3].to_string());
                original_key_slice = cap[2].to_string().substring(0,15).to_string();
            }

            let mut rng = rand::thread_rng();
            let random_key = format!("{}{}{}{}{}{}{}{}{}", rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10));
            reader.replace(format!("{}{}{}", key_regex[0], key_regex[1], key_regex[2]).as_str(), format!("{}{}{}{}", key_regex[0], original_key_slice, random_key, key_regex[2]).as_str());
            reader.to_file(application_file);
        }   
            println!("Successfully Patched {}", self.folder_name);
        } else {
            println!("Couldn't Patch {}", self.folder_name);
        }
    }
}

fn main() {
    if !windows::is_app_elevated() {
        println!("This program needs to be run as admin!");
        pause();
        std::process::exit(0x0100);
    }

    let mut dir = String::new();
    println!("Enter your Adobe Directory: ");
    let _=stdout().flush();
    stdin().read_line(&mut dir).expect("Did not enter a correct string");

    let paths = fs::read_dir(dir.replace('\r', "").replace('\n', "")).unwrap();

    let mut adobe_products = vec![];

    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            if String::from(path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap()).contains("Adobe") {
                let mut product = AdobeProduct::new(
                    &path.as_ref().unwrap().path().display().to_string(),
                    &path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_string()
                );
                product.merge_application_files(get_application_files(&product.path));
                adobe_products.push(product);
            }
        }
    }

    if adobe_products.len() == 0 {
        println!("Found no Adobe Products");
    } else {
        println!("Select the Adobe Product where you want to reset your trial:");
        for (i, product) in adobe_products.iter().enumerate() {
            println!("{}) {}", i+1, product.folder_name);
        }
        println!("{}) Reset All", adobe_products.len() + 1);

        let mut choice_str = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut choice_str);

        let choice_nbr: usize = choice_str.trim().parse().unwrap();

        if choice_nbr == adobe_products.len() + 1 {
            for mut adobe_product in adobe_products {
                adobe_product.patch_trial();
            }
        } else {
            adobe_products[choice_nbr - 1].patch_trial();
        }        
        pause();
    }
}

fn get_application_files(dir: &str) -> Vec<String>{
    let paths = fs::read_dir(dir).unwrap();
    let mut application_files:Vec<String> = vec![];

    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            application_files.extend(get_application_files(&path.as_ref().unwrap().path().display().to_string()).iter().cloned());
        } else {
            if path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_string() == "application.xml" {
                application_files.push(path.as_ref().unwrap().path().display().to_string());
            }
        }
    }

    application_files
}

fn pause() {
    let mut dir = String::new();
    println!("Press ENTER to exit...");
    let _=stdout().flush();
    stdin().read_line(&mut dir).expect("");
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
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
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
