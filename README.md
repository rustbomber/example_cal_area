# example_cal_area

substrate入门第4课作业 

实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

```rust
fn main() {
    let circular = Circular { r: 4.0 };
    let circular_area = cal_area(circular);
    assert_eq!(circular_area, PI * 4.0 * 4.0);
    println!("the circular area is: {}", circular_area);

    let triangle = Triangle {
        height: 3.0,
        width: 4.0,
    };
    let triangle_area = cal_area(triangle);
    assert_eq!(triangle_area, 3.0 * 4.0 / 2.0);
    println!("the triangle area is: {}", triangle_area);

    let square = Square {
        height: 12.0,
        width: 15.0,
    };
    let square_area = cal_area(square);
    assert_eq!(square_area, 12.0 * 15.0);
    println!("the square area is: {}", square_area);
}
```

![](https://github.com/rustbomber/example_cal_area/blob/202227d72030443e83e3b13dca8d50d7ff55ee20/screen.png)