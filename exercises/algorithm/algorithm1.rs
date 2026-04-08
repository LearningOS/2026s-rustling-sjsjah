/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
//对LinkedList<T>实现Default特征,default实现节点逻辑构建

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr: Option<NonNull<Node<T>>> = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }
    /*
    1.  定义可变节点node
    2.  将node的下一个节点设为空
    3.  定义node指针,  Option<NonNull<Node<T>>>  
    4.  Box::into_raw(node) —— 剥夺所有权,接管操作权限,node将不会在自动回收该内存,
        同时会获取原始地址[into_raw消耗掉了Box智能指针,node不会再因为box自动释放内存]
    5.  new_unchecked 表示“不检查是否为 0”.
    6.  然后放入option枚举[用some包装]

    7.  分情况匹配:
        (1)如果是none就说明该链表当前为空,设定链表起点
        (2)若有值存在则 用as_ptr()函数拆掉 NonNull 包装转换回 Rust 的 原始指针（*mut Node<T>）
           把旧的最后一个节点的 next 从 None 改成 node_ptr（新车厢地址）
    
    8.  更新尾部指针
        链表长度加一
    */

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /*
    访问函数
    get:封装
    get_ith_node:
    匹配
     1.没有节点了就是none,返回空
     2.有节点
      匹配
        1.计数为零,就是这里as_ptr获取地址,.val获取其值
        2.节点非零,用as_ptr获取地址访问下一个,计数减一
    
    
    */
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where 
        T:PartialOrd
	{
		//TODO
        //先处理边界问题:考虑其中一个是空的情况,直接返回另一个
        if list_a.start.is_none(){return list_b;}
        if list_b.start.is_none(){return list_a;}
		

        let mut res = LinkedList::new();
        let mut  p1 = list_a.start;
        let mut  p2 = list_b.start;
        
        //需要一个临时的尾指针来确定当前末尾
        let mut last_added: Option<NonNull<Node<T>>>= None;

        //不安全声明开始进行比较操作
        unsafe{
            while p1.is_some() && p2.is_some() {
                //拿出头指针: .unwrap() 拆掉 p1/p2 的Option
                let p1_ptr = p1.unwrap();
                let p2_ptr = p2.unwrap();

                //被选中变量
                let chosen : NonNull<Node<T>>;
                //如果p1<=p2则将p1放入chosen,反之将p2放入,之后将其下一个放入 p1/p2
                if (*p1_ptr.as_ref()).val<=(*p2_ptr.as_ref()).val{
                    chosen = p1_ptr;
                    p1 = (*p1_ptr.as_ref()).next;
                }
                else {
                    chosen = p2_ptr;
                    p2 = (*p2_ptr.as_ref()).next;
                }

                //如果res链表为空则将元素设为开始,如果有值存在:将值放入末尾的下一个
                match last_added {
                    None=>{res.start = Some(chosen)}
                    Some(prev)=>{(*prev.as_ptr()).next = Some(chosen)} 
                }//将值放至结尾  长度+1
                last_added=Some(chosen);
                res.length+=1;
            }
            

            //将未进入res链表的值赋给rest
            let rest = if p1.is_some(){p1}else{p2};
            if let Some(prev) = last_added {
            (*prev.as_ptr()).next = rest;
            }
        
        //更新指针,end出现了变化
        let mut temp = rest;
        if rest.is_none(){
            res.end = last_added;
        }else {
            while let Some(node) = temp {
                res.end = Some(node);
                res.length +=1; 
                temp = (*node.as_ptr()).next;
            }
        }

        
        }
        
     let mut a = list_a; // 只是为了明确语义
        let mut b = list_b;
        a.start = None; a.end = None;
        b.start = None; b.end = None;

        res
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}
/*是空就ok,非空 */

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

/*
1.write! 是一个宏（Macro），它的作用是 “向指定的目的地写入格式化字符串:把 val 转换成字符串。”   (ps:write! 的灵活性：它可以把东西印到任何实现了 Write 特征的地方。在 fmt 函数里，这个目的地就是参数 f（它是内存里的一个缓冲区）。)
2.使用as_ref()将被NonNull包装的指针合法化
  NonNull 是个包装：它本质上只是一个 数字（内存地址）。Rust 的编译器非常保守，它不准你直接对一个“数字”进行操作，因为它不知道那个地址上的内存是否安全。
  as_ref() 是转换器：它把这个“冷冰冰的数字地址”转换成 Rust 认可的 引用（&Node<T>）
3.注意!!!在到达最后前所有节点的程序其实都没完成
4.在取完值后
    struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>, // 指向下一个节点
    }
5.write!实现的递归调用
    write! 在递归里的“传送门”作用
    write!(f, "{}, {}", self.val, unsafe { node.as_ref() })
    准备阶段：write! 准备往 f（缓冲区）里写东西。
    填第一个空：它把 self.val（比如数字 1）写进去。
    遇到第二个空：它看到 node.as_ref()（指向 Node B 的地址）。
    开启传送门：重点!!!！ write! 发现要填这个空，必须先知道 Node B 长什么样。于是它原地发起了一个新的调用，跳到了 Node B 的 fmt 函数里。
    嵌套循环：在 Node B 的函数里，又遇到了 write!，又看到了 Node C 的地址……
    所以，write! 的性质就是：只要还没到 None，它就会为了填满那个 {} 而不断地去敲下一个节点的大门。

*/

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}