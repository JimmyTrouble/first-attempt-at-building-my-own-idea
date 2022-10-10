// creat struct holds character traits (size, race, skill)
// create enum that holds info on race and skil
// create a character using let (size, race, skill), (team)
// print character to console using impl

//level up char


// create enum that holds info on race and skil
// #[derive(Debug)]
// enum Race {
//     human,
//     elf,
//     dwarf   
// } 

// // create enum that holds info on race and skil
// #[derive(Debug)]
// enum Skill{
//     warrior,
//     Gurthu_warrior,

//     mage,
//     Slutty_mage,

//     tech,
//     Swol_Brained_Tech  
// }

// // creat struct holds character traits (size, race, skill)
// struct Character_Traits{
//     size: i32,
//     race: Race,
//     skill: Skill
// }

// // print character to console using impl
// impl Character_Traits {

//     fn new(size: i32, race:Race, skill:Skill) -> Self {
//         Self { size, race, skill}       
//     }
//     fn leve_up(&self) -> Skill {
//         Skill::Gurthu_warrior
        
//     }
// }

// // print character to console using impl
// //level up char
// fn main() {
//     let mut Human_warrior = Character_Traits::new(6, Race::human , Skill::warrior);
//     println!("{:?}, {:?}, {:?}", Human_warrior.race, Human_warrior.size, Human_warrior.skill); 
    
//     Human_warrior.skill = Human_warrior.leve_up();
//     println!("{:?}, {:?}, {:?}", Human_warrior.race, Human_warrior.size, Human_warrior.skill); 

//     let mut Elf_Mage = Character_Traits::new(5, Race::elf , Skill::mage);
//     println!("{:?}, {:?}, {:?}", Elf_Mage.race, Elf_Mage.size, Elf_Mage.skill); 
    
//     Elf_Mage.skill = Elf_Mage.leve_up();
//     println!("{:?}, {:?}, {:?}", Elf_Mage.race, Elf_Mage.size, Elf_Mage.skill); 
// }


#[derive(Debug)]
enum Race {
    human,
    elf,
    dwarf   
} 

// create enum that holds info on race and skil
#[derive(Debug)]
enum Skill{
    warrior,
    Gurthu_warrior,

    mage,
    Slutty_mage,

    tech,
    Swol_Brained_Tech  
}

// creat struct holds character traits (size, race, skill)
struct Character_Traits{
    size: i32,
    race: Race,
    skill: Skill
}

// print character to console using impl
impl Character_Traits {

    fn new(size: i32, race:Race, skill:Skill) -> Self {
        Self { size, race, skill}       
    }
    fn leve_up(&self) -> Skill {

        match  self.skill {

            Skill::warrior => Skill::Gurthu_warrior,
            Skill::Gurthu_warrior => Skill::Gurthu_warrior,
            Skill::mage => Skill::Slutty_mage,
            Skill::Slutty_mage => Skill::Slutty_mage,
            Skill::tech => Skill::Swol_Brained_Tech,
            Skill::Swol_Brained_Tech => Skill::Swol_Brained_Tech            
        }       
    }
}

// print character to console using impl
//level up char
fn main() {

    //create
    let mut Human_warrior = Character_Traits::new(6, Race::human , Skill::warrior);
    println!("{:?}, {:?}, {:?}", Human_warrior.race, Human_warrior.size, Human_warrior.skill); 

    //level up
    Human_warrior.skill = Human_warrior.leve_up();
    println!("{:?}, {:?}, {:?}", Human_warrior.race, Human_warrior.size, Human_warrior.skill); 



    //create
    let mut Elf_Mage = Character_Traits::new(5, Race::elf , Skill::mage);
    println!("{:?}, {:?}, {:?}", Elf_Mage.race, Elf_Mage.size, Elf_Mage.skill); 
 
    //level up
    Elf_Mage.skill = Elf_Mage.leve_up();
    println!("{:?}, {:?}, {:?}", Elf_Mage.race, Elf_Mage.size, Elf_Mage.skill); 
}