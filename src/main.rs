use std::io::{self, Write};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use std::thread;
use std::sync::{Arc,Mutex};
// use std::time::Duration;

fn main() {
    let mut age = 10;
    println!("Hello, crabby! age {}", age);   
    age = 15;
    println!("Hello, crabby! age {}", age);

    let weight = 63.5;
    let messgae1 = "My".to_string();
    let messgae2 = String::from("weight");

    let msg = format!("{} {} is", messgae1, messgae2);

    println!("{} {}",msg, weight as i32);




    let mut input = String::new();

    println!("How about weather today?");
    print!("(sunny, rainy, stromy) : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let weather = input.trim().to_lowercase().to_string();

    if weather == "sunny"{
        println!("Today Sunny ... Crabby will cross the river by swiming!");
    } else if weather == "rainy"{
        println!("Today Rainny ... Crabby will build a bridge to stay dry");
    } else if weather == "stormy" {
        println!("Today Stormy ... Crabby will wait for better weather");
    } else {
        println!("Crabby sleep zZzZZ");
    }


    let mut input = String::new();

    println!("We encounters enemy !!!");
    println!("1. Goblin");
    println!("2. Troll");
    println!("3. Dragon");
    print!("choose number for attck :");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Match

    let enemy :i32 = input.trim().parse().unwrap_or(0);

    match enemy {
        1 => println!("Crabby use Sword attck"),
        2 => println!("Crabby set Trap"),
        3 => println!("Crabby cast magic"),
        _ => println!("Crabby run away.."),
    }

    // Loop

    let mut wood = 0;
    'collect_wood: loop {
        wood += 1;
        println!("got wood +1 Now have {}",wood);
        if wood > 10 {
            break 'collect_wood;
        }
    }

    let new_str = merge_string("Hello".to_string(), "Crebby".to_string());
    println!("{}",new_str);



    // Ownership and Borrow ----- ***specific Heap memory
    println!("====================================================");


    let crabby_master = "Crabby Master".to_string();

    // =====================
    // let crabby_01 = crabby_master;
    // println!("{}",crabby_01);
    // println!("{}",crabby_master); // error because crabby_01 got Ownership

    let crabby_02 = crabby_master.clone();
    println!("{} was Clone by Crabby02",crabby_02);
    println!("I'm {} OG",crabby_master);


    let crabby_03 = &crabby_master;
    println!("{} was Borrowing by Crabby03",crabby_03);
    println!("I'm {} OG",crabby_master);
    
    
    // =====================

    let mut crabby_master_mut = "Crabby Master mutable".to_string();
    let crabby_04 = &mut crabby_master_mut;

    // println!("{} was Borrowing by Crabby04",crabby_04);
    // println!("I'm {}",crabby_master_mut);

    *crabby_04 = "Crabby High Grand Master".to_string();
    println!("Crabby04 change value to {}",crabby_04);
    // println!("I'm {}",crabby_master_mut); // error bacause Ownership transfer to Crabby 04

    crabby_04.push_str(" Elite");
    println!("I'm {} ... crabby04", crabby_04);

    println!("I'm Original {}",crabby_master_mut); // can execute bacause Crabby 04 finish ... memory was destroyed

    // =====================
    
    // Global anad Local

    let mut golem = "Golem2476".to_string(); // global
        { // local 
            let clone_golem_01 = &mut golem;
            // println!("Master {}",golem);
            println!("Clone {} ... was modify",clone_golem_01);
            clone_golem_01.push_str(" Special Edition");
            println!("Clone evolution to.... {}",clone_golem_01);
        }
    println!("Master {}",golem);


    // Lifetime use <'a> need to use lifetime because we return output from input that mean memory may was killed before was return

    let map1 = "Ancient Map of the Sea";
    let map2 = "Map to Hidden Gold";
    let chosen_map = longest_map(map1, map2);
    println!("Crabby's longest map: {}", chosen_map);

    // struct ------------------ start

    let mut player1= Crabby {
        name: "Plug".to_string(),
        skill_level: 10,
        hit_points: 100,
    };

    let mut player2= Crabby {
        name: "Lemon".to_string(),
        ..player1 // beware copy Heap, it not clone it will move owner
    };

    player1.take_damage(150);
    player2.take_damage(46);
    player2.skill_lvup();
    player2.got_healing(20);   
    
    // struct ------------------ end

    // enum ------------------ start

    let fighting = Crabbystate::Fighting;
    let collecting = Crabbystate::Collecting(20);
    let defending = Crabbystate::Defending;

    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();

    // enum ------------------ end

    // Trait ------------------ start

    let gold = Inventory {item: 100};
    let sword = Inventory {item: "Excalibur"};

    gold.display();
    sword.display();

    // Trait ------------------ end

    // String vs borrow String (&str) ------------------ start

    let map = String::from("Old Map");

    // let borrow_map = &map[0..5];
    // println!("{}",borrow_map)
    let borrow_map = map.as_str();

    let mut crabby_map = borrow_map.to_string();

    println!("{}",crabby_map);
    crabby_map.push_str(" to new map");
    println!("{}",crabby_map);
    // String vs borrow String (&str) ------------------ end

    // for loop
    for i in 0..5 {
        println!("for loop : {}",i)
    }

    let treasures: [&str;4] = ["Gold","Silver","Ruby Gem","Emerald"];
    let mut energy = 2;

    for treasure in treasures.iter(){
        // println!("{}",treasure);

        if energy == 0 {
            println!("Not enough energy");
            break;
        } else if treasure == &"Ruby Gem"{
            println!("Lucky you got {}", treasure);
            break;
        }

        energy -= 1;

    }

    
    // Vector ------------------ start
    // create an empty Vector
    let mut treasures: Vec<String> = Vec::new();

    // add some treasures to vector
    treasures.push(String::from("Gold coins"));
    treasures.push(String::from("Emerald"));
    treasures.push(String::from("Sword"));

    println!("Crabby's Treasures {:?} lenght of Treasure is {}",treasures, treasures.len());
    println!("Capacity of Treasure is {}",treasures.capacity());

    let one_treasure = &treasures[0];
    println!("borrow First : {}",one_treasure);

    let last_treasure = treasures.pop();
    let second_treasure = treasures.remove(1);

    println!("pop remove Last : {:?}",last_treasure);
    println!("Remove index Second : {}",second_treasure);

    println!("Crabby's Treasures {:?} lenght of Treasure is {}",treasures, treasures.len());
    println!("Capacity of Treasure is {}",treasures.capacity());

    treasures.shrink_to(2);
    println!("Adjust capacity to 2, Capacity of Treasure is {}",treasures.capacity()); 

    // Vector  ------------------ end

    // iterators and Closures ------------------ start

    let treasures = vec![100,200,300,400];

    let double_treasure: Vec<i32> = treasures.iter().map(|x| x * 2).collect();

    println!("{:?}",double_treasure);

    // iterators and Closures ------------------ end

    // HashMap ----- (Key, Value) ------------------ start

    let mut treasures = HashMap::new();

    treasures.insert("Gold Coin", 10);
    treasures.insert("Gem", 1000);

    if let Some(gem) = treasures.get("Gem") {
        println!("Gem: {}", gem);
    }
    
    if let Some(gold) = treasures.get_mut("Gold Coin")  {
        *gold += 100;
        println!("Gold Coin + 100, Now is {}", gold)
    }

    for (k, v) in treasures.iter() {
        println!("key is {}, Value is {}", k, v);
    }

        // tuple in vector to Hashmap
    let items = vec![("Gold", 50), ("Silver", 100)];
    let treasure_map: HashMap<&str, i32> = items.into_iter().collect();
    println!("{:?}", treasure_map);  // Output: {"Gold": 50, "Silver": 100}
        // tuple in vector to Hashmap

    // HashMap ----- (Key, Value)  ------------------ end

    // Error Handling  ------------------ start

    // ---------------------- Generic Type Option ----
    // Option Type --- Option ค่าจะมีหรือไม่มีก็ได้ Some คือ มี Value, None คือ ไม่เจอ Value

    // ------------------------------- เขียนแบบ function
    fn open_box(is_empty:bool) -> Option<String> {
        if is_empty {
            None
        } else {
            Some("You found Apple".to_string())
        }
    }

    let item_in_box = match open_box(false) {
        Some(t) => t,
        None => "No item Found".to_string(),
    };

    println!("{}",item_in_box);

    // -------------------------------

    let treasures = vec!["Gold", "Ruby", "Emerald"];
    let treasure = match treasures.get(3) {
        Some(t) => t,
        None => "No Treasure Found",
    };

    println!("{}", treasure);

    // ---------------------- Generic Type Result ----
    // Result <__type เมื่อ Ok__, __type เมื่อ Error__>
    fn open_chest(is_locked:bool) -> Result<String, String> {
        if is_locked {
            Err("Treasure is locked".to_string())
        } else {
            Ok("Crabby open Treasure and found Gold !".to_string())
        }
    }
    let chest_result = open_chest(true);
    match chest_result {
        Ok(message) => println!("{}",message),
        Err(error) => println!("error : {}",error),
    }

    // Unwrap_or คือการบอกว่าถ้า error ค่า default จะเป็นอะไร
    let gem = treasures.get(3).unwrap_or(&"Gold");
    println!("gem parameter : {}", gem);

    // Unwrap_or_else 

    // การใช้ ? กรณีที่ Result Error เหมือนกันกับ Function ที่ใหญ่กว่า
    // () คือ void แสดงถึงไม่มีข้อมูล ไม่มีค่า
    fn result_error() -> Result<(), String> {
        let chest_result = open_chest(false)?; // ใช่ ? ต่อท้าย จะเป็นรูปแบบย่อในการดัก error 
        println!("message in 'chest_result' function: {}",chest_result);
        Ok(()) // คืนค่า Ok ไม่มีข้อมูลเพิ่มเติม
    }

    match result_error() {
        Ok(()) => {}, // ไม่ทำอะไรเลยในกรณี Ok
        Err(error) => println!("error: {}",error),
    }

    // การใช้ ? กรณีที่ Result Error เหมือนกันกับ Function ที่ใหญ่กว่า

    // Error Handling ------------------ end

    // Smart Pointer ------------------ start
    // Box<T> คือการทำกล่องเก็บเพื่อข้อมูลใน Heap 8 bytesให้ Rust คำนวณ memory ตอน compline ได้
    // Rc คือทำให้เราสามารถยืม heap ได้หลาย owner (หลายตัวแปร เข้าถึง ข้อมูลเดียวกัน)
    // RefCell<T> ให้ Smart Pointer เป็น mutable
    // use std::{cell::RefCell, rc::Rc};
        // ---------- test
    let gold = Box::new(10); //สร้าง กล่อง heap
    let epic_loot = Rc::new(RefCell::new(gold)); // สร้างตัวแปรมาเพื่อให้ point ได้หลาย owner (Rc), และ mutable (RefCell)

    let loot_1 = Rc::clone(&epic_loot);
    let loot_2 = Rc::clone(&epic_loot);

    println!("epic loot variable is {}",epic_loot.borrow());

    **loot_1.borrow_mut() += 20; // * แรก เข้าถึง Rc,RefCell, * สอง เข้าถึง Box
    **loot_2.borrow_mut() += 100;

    println!("epic loot variable is {}",epic_loot.borrow());   
        // ---------- test

    // Smart Pointer ------------------ end

    // Traits as Types ------------------ start
    // ถ้ามี item หลายแบบ แล้วทำงานเหมือนกัน แต่ไส้ในต่างกัน ใช้ Trait เป็น Type แทน Struct ได้เลย
        //static
    println!("---------Trait as type Static---------");
    let crabby_sword = Sword;
    let crabby_bow = Bow;    
    let crabby_potion = Potion;

    use_gear_static(crabby_sword);
    use_gear_static(crabby_bow);
    use_gear_static(crabby_potion);

        //dynamic
    println!("---------Trait as type Dynamic---------");
    let crabby_sword = Box::new(Sword);
    let crabby_bow = Box::new(Bow);    
    let crabby_potion = Box::new(Potion);

    use_gear_dynamic(crabby_sword);
    use_gear_dynamic(crabby_bow);
    use_gear_dynamic(crabby_potion);
    // Traits as Types ------------------ end

    // Threading ------------------ start
    // use std::thread
    let threaded = thread::spawn(|| {
        println!("Crabby is mixing a potion!");
    });

    threaded.join().unwrap();
        /*
        threaded.join().unwrap(); จะบล็อกการทำงานของ thread หลักจนกว่า thread ย่อยจะเสร็จสิ้น
        ถ้าไม่ต้องการรอให้ thread ย่อยทำงานเสร็จ สามารถเรียก spawn และปล่อยให้มันทำงานแบบแยกจากกันโดยไม่ต้องใช้ join:

        สรุป
        ใช้ join ถ้าคุณต้องการให้แน่ใจว่า thread ย่อยทำงานเสร็จก่อนที่จะดำเนินการต่อ (เช่นในกรณีที่ต้องการผลลัพธ์จาก thread ย่อย)
        ไม่ใช้ join หากต้องการให้ thread ย่อยทำงานไปพร้อม ๆ กับ thread หลัก (background task)
         */
        
        // smart pointer สำหรับ thread 
        // use std::sync::Arc; เหมือนกับ RC ที่แค้ Arc ทำให้มีหลาย owner ใน หลายๆ thread
        // move คือ pass owner เข้าไปใน thread เลย ให้ thread เป็น owner ไปเลย
        // Mutex --- use std::sync::Mutex; เหมือน RefCell แต่ใช้ได้กับ multi thread
        // Mutex สามารถ .lock() ไว้ก่อน เพื่อให้ thread แก้ค่าเสร็จก่อน thread ต้องรอ อื่นถึงจะแก้ไขค่าได้  
        // smart pointer สำหรับ thread 

        // example---------start

    let crabby_gold = Arc::new(Mutex::new(10));

    // สร้าง loot_1 แล้ว clone crabby_gold + move owner ไปแล้ว เปลี่ยนค่า
    let loot_1 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 200;
        }
    });

    let loot_3 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 80;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Gold: {}", crabby_gold.lock().unwrap());
        // example---------end

    // Threading ------------------ end

}

// use for Traits as Types ------------------ start
trait Gear {
    fn use_gear(&self);
}
struct Sword;
struct Bow;
struct Potion;

fn use_gear_static<T: Gear>(item: T) {
    item.use_gear();
}

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire Arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink Potion");
    }
}
    // Dynamic ----- start
fn use_gear_dynamic(item: Box<dyn Gear>) {
    item.use_gear();
}
    // Dynamic ----- end


// use for Traits as Types ------------------ end

//======================= trait
// T เป็น Generic Type Parameter สามารถเป็น type อะไรก็ได้เช่น i32, &str, string
struct Inventory <T>{
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T> where T: std::fmt::Debug{
    fn display(&self) {
        println!("{:?}", self.item);
    }

} 

//======================= trait

//======================= enum
enum Crabbystate {
    Fighting,
    Collecting(u32),
    Defending,
}

impl Crabbystate {
    fn state_represent(&self){
        match self {
            Crabbystate::Fighting => println!("Fighting Mode"),
            Crabbystate::Collecting(amount) => println!("collecting item {} ea", amount),
            Crabbystate::Defending => println!("Dedend mode"),
        }
    }
}

//======================= enum

//======================= struct + method
struct Crabby {
    name: String,
    skill_level: u32,
    hit_points: u8,
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        // self.hit_points -= damage;
        self.hit_points = self.hit_points.saturating_sub(damage); //use this method protect overflow value if value < 0 is not error
        println!(
            "{} take {} damage, hit point now at {}", self.name, damage, self.hit_points 
        );
    }

    fn got_healing(&mut self, healing: u8) {
        self.hit_points += healing;
        println!(
            "{} was healing {}, hit point now at {}", self.name, healing, self.hit_points
        );
    }

    fn skill_lvup(&mut self) {
        self.skill_level += 1;
        println!(
            "{} skill lv up to {}", self.name, self.skill_level
        );
    }
}

//======================= struct + method

fn merge_string (string1: String, string2: String) -> String {
    return format!("{} {}", string1, string2)
}

fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
    map1
    } else {
    map2
    }
}