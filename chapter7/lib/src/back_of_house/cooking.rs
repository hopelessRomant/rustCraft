fn cook_order() {
// --snip--
// absolute path
crate::front_of_house::serving::deliver_order();

//relative path
super::super::serving::deliver_order();
}