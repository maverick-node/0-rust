pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
      let mut my_vec: Vec<Box<u32>> = Vec::new();

      let mut vc:Vec<f64> = Vec::new();
    
      for i in s.split_whitespace(){
        if i.contains('k'){
            let rr: Vec<_>= i.split('k').collect();
            let prs = rr[0].parse::<f64>().unwrap();
            let com = prs*1000.;
            let pop = com as u32;
            my_vec.push(Box::new(pop));

        }else{
             let prs = i.parse::<u32>().unwrap();
             my_vec.push(Box::new(prs));


        }
      }
my_vec
}

pub fn into_unboxed(a: Vec<Box<u32>>)-> Vec<u32>{
    let mut vecc:Vec<u32>= Vec::new();
 for i in a{
   vecc.push(*i);
 }
 vecc
}