use std::{fs::{self, File},io::Write};

pub struct Range {
    start:u32,
    end:u32,
}

pub struct Route {
    vec:Vec<Range>
}

impl Route {
    pub fn new() -> Route {
        Route { vec:Vec::new() }
    }
    
    pub fn init(&mut self) {
        let mut f = File::options().append(true).open("k.txt").unwrap();
        let content = fs::read_to_string("chnroutes.txt").unwrap();
        let vec:Vec<&str> = content.split("\n").collect();
        for v in vec {
            let s = self.line_to_range(v);
            f.write(s.as_bytes()).unwrap();
        }
        
    }

    pub fn hit(&self,ip:&str) -> bool {
        let num = Route::ip_to_number(ip);
        for v in &self.vec {
            if num >= v.start && num <= v.end {
                return true
            }
        }
        false
    }

    fn line_to_range(&mut self,line:&str) -> String {
        let vec:Vec<&str> = line.split("/").collect();
        if vec.len() <= 1  { return "".into(); }
        let start = Route::ip_to_number(vec[0]);
        let mask:u32 = vec[1].parse().unwrap();
        let end = start + 2u32.pow(32-mask);
        self.vec.push( Range{ start,end } );
        format!("{},{}\n",start,end)
    }

    fn ip_to_number(ip:&str) -> u32 {
        let mut ret:u32 = 0;
        let mut e:u32 = 4;
        let vec:Vec<&str> = ip.split('.').collect();
        for v in vec {
            e = e - 1;
            match v.parse::<u32>() {
                Ok(n) => {
                    if n > u8::MAX as u32  { return 0; }
                    ret = ret + n*256u32.pow(e);
                }
                _ => { return 0; }
            }
        }
        ret
    }
}