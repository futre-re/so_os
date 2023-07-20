use crate::{
    print, println,
    vga_buffer::{self, BUFFER_WIDTH},
};
pub fn start_show() {
    let coin_os = r"

      _______  _____  
      |______ |     |      
      ______| |_____|      
                                                                                               
                _______ __   __ _______ _______ _______ _______
                |______   \_/   |______    |    |______ |  |  |
                ______|    |    ______|    |    |______ |  |  |
                                                                          
    ";
    let os_name = "HuChangjing";
    let time_of_making = "2023/6/28";
    let copyright_notice = "Copyright2023HuChangjing.AllRightsReserved.";
    let notice = "If you have any questions, 
    please contact me at this email address:futurehadt@outlook.com";
    println!("{coin_os}");
    println!("Fabricator:{}", os_name);
    println!("Time Of Making:{}", time_of_making);
    println!("Copyright Information:{}", copyright_notice);
    println!("Notice:{}", notice);
    //打印分割
    for i in 0..BUFFER_WIDTH - 10 {
        print!("-");
    }
    println!();
}
