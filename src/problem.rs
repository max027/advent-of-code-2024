use std::collections::HashMap;
use std::fs;
pub fn day1_part1()->i32{
    let content=fs::read_to_string("C:\\Users\\saurabh\\programming\\rust\\advent\\src\\input1.txt").expect("file not found");
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
    let content=fs::read_to_string("C:\\Users\\saurabh\\programming\\rust\\advent\\src\\input1.txt").expect("file not found");
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
