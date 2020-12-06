use std::io::Read;
use regex::RegexBuilder;

// Fetch web content by HTTP GET
fn http_get(url: &String) -> Result<String, String> {
    let mut res = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| e.to_string())?;
    Ok(body)
}

// Read ocv_define_module or ocv_add_module 
fn get_dependencies_line(body: &String) -> Option<String> {
    let re = RegexBuilder::new(r"ocv_(define|add)_module\((?P<modules>.+?)\)$")
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let captures = re.captures(&body);
    match captures {
        Some(c) => return Some(c.name("modules").unwrap().as_str().replace("\n", " ").to_owned()),
        None => None,
    }
}

// Parse ocv_define(add)_module line string
fn parse(dependencies_string: &String) -> (&str, Vec<String>, Vec<String>)
{
    let tokens: Vec<&str> = dependencies_string.split_whitespace().collect();
    let module_name = tokens[0];

    let required: Vec<_> = tokens[1..].iter()
        .take_while(|&s| s != &"WRAP" && s != &"OPTIONAL" && s != &"PRIVATE_REQUIRED" && s != &"#")
        .filter(|&s| !s.starts_with("$"))
        .filter(|&s| s != &"REQUIRED" && s != &"OPTIONAL" && s != &"INTERNAL") 
        .map(|&s| s.replace("opencv_", ""))
        .collect();

    let optional: Vec<_> = tokens[1..].iter()
        .skip_while(|&s| s != &"OPTIONAL")
        .take_while(|&s| s != &"WRAP" && s != &"PRIVATE_REQUIRED" && s != &"#")
        .filter(|&s| !s.starts_with("$"))
        .filter(|&s| s != &"OPTIONAL") 
        .map(|&s| s.replace("opencv_", ""))
        .collect();

    (module_name, required, optional)
}

fn main() {
    //*
    let modules = [
        "calib3d",
        "core",
        "dnn",
        "features2d",
        "flann",
        "gapi",
        "highgui",
        "imgcodecs",
        "imgproc",
        "imgproc",
        //"java",
        //"js",
        "ml",
        //"objc",
        "objdetect",
        "photo",
        //"python",
        "stitching",
        "ts",
        "video",
        "videoio",
    ];
    //*/
    /*
    let modules = [
        "alphamat",
        "aruco",
        "bgsegm",
        "bioinspired",
        "ccalib",
        "cnn_3dobj",
        "cudaarithm",
        "cudabgsegm",
        "cudacodec",
        "cudafeatures2d",
        "cudafilters",
        "cudaimgproc",
        "cudalegacy",
        "cudaobjdetect",
        "cudaoptflow",
        "cudastereo",
        "cudawarping",
        "cudev",
        "cvv",
        "datasets",
        "dnn_objdetect",
        "dnn_superres",
        "dnns_easily_fooled",
        "dpm",
        "face",
        "freetype",
        "fuzzy",
        "hdf",
        "hfs",
        "img_hash",
        "intensity_transform",
        //"julia",
        "line_descriptor",
        //"matlab",
        "mcc",
        "optflow",
        "ovis",
        "phase_unwrapping",
        "plot",
        "quality",
        "rapid",
        "reg",
        "rgbd",
        "saliency",
        "sfm",
        "shape",
        "stereo",
        "structured_light",
        "superres",
        "surface_matching",
        "text",
        "tracking",
        "videostab",
        "viz",
        "xfeatures2d",
        "ximgproc",
        "xobjdetect",
        "xphoto",
    ];
    */

    println!("digraph \"OpenCV Dependencies\" {{");
    println!("  graph [");
    println!("    rankdir=RL");
    println!("  ]");

    for module in modules.iter() {
        let url = format!("https://raw.githubusercontent.com/opencv/opencv/master/modules/{}/CMakeLists.txt", module);
        //let url = format!("https://raw.githubusercontent.com/opencv/opencv_contrib/master/modules/{}/CMakeLists.txt", module);

        let body = match http_get(&url) {
            Ok(b) => b,
            Err(e) => {println!("{}", e); continue}
        };
        //println!("body: {}", body);
        let dependencies_string = match get_dependencies_line(&body) {
            Some(ds) => ds,
            None => continue
        };

        let (module_name, required, optional) = parse(&dependencies_string);
        if !required.is_empty() {
            println!("  \"{}\" -> {{ {} }}", 
                module_name, 
                required.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" "));
        }
        if !optional.is_empty() {
            println!("  \"{}\" -> {{ {} }} [style=\"dashed\"]", 
                module_name, 
                optional.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" ")); 
        }       
    } 

    println!("}}");
}
