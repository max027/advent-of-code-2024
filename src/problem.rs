use std::collections::HashMap;
use std::fs;
pub fn day1_part1()->i32{
    let content=fs::read_to_string("../input1.txt").expect("file not found");
    let mut list1:Vec<i32>=vec![]; 
    let mut list2:Vec<i32>=vec![]; 
    let mut  ans=0;

    content.lines().for_each(|line|{
        let num:Vec<&str>=line.split_whitespace().collect();
        list1.push(num[0].parse().unwrap());
        list2.push(num[1].parse().unwrap());
    });

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

    content.lines().for_each(|line| {
        let num:Vec<&str>=line.split_whitespace().collect();
        list1.push(num[0].parse().unwrap());
        list2.push(num[1].parse().unwrap());
    });   

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

/*   not needed  
 *    fn check_safty(line:Vec<i32>)->bool{
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
*/
    let content=fs::read_to_string("../input2.txt").expect("file not found");
    let mut ans=0;
    content.lines().for_each(|i|{
        let int_vec:Vec<i32>=i.split_whitespace().filter_map(|n| n.parse().ok()).collect();
        let is_inc=int_vec.windows(2).all(|line| (line[0]<=line[1])&& ((line[1]-line[0]).abs()>=1 && (line[1]-line[0]).abs()<=3)); 

        let is_dec=int_vec.windows(2).all(|line| (line[0]>=line[1])&& ((line[1]-line[0]).abs()>=1 && (line[1]-line[0]).abs()<=3)); 

        if is_inc||is_dec {
            ans+=1;
        }
    });

    ans
}

pub fn day2_part2(){
    fn is_safe(line:&Vec<i32>)->bool{

        let is_inc=line.windows(2).all(|line| (line[0]<=line[1])&& ((line[1]-line[0]).abs()>=1 && (line[1]-line[0]).abs()<=3)); 

        let is_dec=line.windows(2).all(|line| (line[0]>=line[1])&& ((line[1]-line[0]).abs()>=1 && (line[1]-line[0]).abs()<=3)); 
        is_inc||is_dec
    }

    fn problem_dampner(line:&Vec<i32>,ans:&mut i32){
        let mut line_vec=line.to_vec();
        for (i,_) in line.iter().enumerate(){
            let removed=line_vec.remove(i);
            if is_safe(&line_vec){
                *ans+=1; 
                line_vec.insert(i, removed);
                return;
            }
            line_vec.insert(i, removed);
        }
    }
    let content=fs::read_to_string("../input.txt").expect("file not found");
    let mut ans=0;
    for i in content.lines(){
       let int_vec:Vec<i32>=i.split_whitespace().filter_map(|n| n.parse().ok()).collect(); 
        if is_safe(&int_vec){
            ans+=1;
            continue;
        }

      problem_dampner(&int_vec, &mut ans); 
    }

    println!("{}",ans);
}



