use std::collections::HashMap;
use std::fs;
pub fn day1_part1()->i32{
    let content=fs::read_to_string("../input1.txt").expect("file not found");
    let mut list1:Vec<i32>=vec![]; 
    let mut list2:Vec<i32>=vec![]; 
    let mut  ans=0;
    for line in content.lines(){
        let num:Vec<&str>=line.split_whitespace().collect();
        list1.push(num[0].parse().unwrap());
        list2.push(num[1].parse().unwrap());
    }
    list1.sort_by(|a,b| a.partial_cmp(b).unwrap());
    list2.sort_by(|a,b| a.partial_cmp(b).unwrap());
    for i in 0..list1.len(){
        ans+=(list2[i]-list1[i]).abs();
    }
    ans
}

pub fn day1_part2()->i32{
    let content=fs::read_to_string("./input1.txt").expect("file not found");
    let mut list1:Vec<i32>=vec![]; 
    let mut list2:Vec<i32>=vec![]; 
    let mut map:HashMap<i32,i32>=HashMap::new();
    let mut  ans=0;
    for line in content.lines(){
        let num:Vec<&str>=line.split_whitespace().collect();
        list1.push(num[0].parse().unwrap());
        list2.push(num[1].parse().unwrap());
    }
   
    for i in list2{
        *map.entry(i).or_insert(0)+=1;
    }

    for i in list1{
        if map.contains_key(&i) {
            if let Some(val)=map.get(&i){
                ans+=i*val;
            }

        }
    }
    ans
}

pub fn day2_part1()->i32{
    fn check_safty(line:Vec<i32>)->bool{
        let mut is_dec=true;
        let mut is_inc=true;
        for i in 1..line.len(){
            match line[i].cmp(&line[i-1]) {
                std::cmp::Ordering::Less=>is_dec=false,
                std::cmp::Ordering::Greater=>is_inc=false,
                _=>return false
            }
            
            if !is_inc && !is_dec{
                return false;
            }
        }
        
        for i in 1..line.len(){
            if (line[i-1]-line[i]).abs()<=3 && (line[i-1]-line[i]).abs()>=1{
                continue;
            }else{
                return false;
            }
        }
        true 
    }
    let content=fs::read_to_string("../input2.txt").expect("file not found");
    let mut ans=0;
    for i in content.lines(){
        let num:Vec<&str>=i.split_whitespace().collect(); 
        let int_vec:Vec<i32>=num.iter().map(|s| s.parse().unwrap()).collect();
        if check_safty(int_vec) {
            ans+=1;
        }
    }

    ans
}

pub fn day2_part2(){
    let content=fs::read_to_string("../input2.txt").expect("file not found");
    

}



