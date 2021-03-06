use obi::Image;

#[test]
fn image_indexing() {
    let (width, height) = (30, 50);
    let img = Image::new(width, height);
    for x in 0..width {
        for y in 0..height {
            assert_eq!((y * width + x) as usize, img.index(x, y).unwrap());
        }
    }
}

#[test]
#[should_panic]
fn x_value_out_of_bounds() {
    let img = Image::new(30, 50);
    img.index(30, 0).unwrap();
}

#[test]
#[should_panic]
fn y_value_out_of_bounds() {
    let img = Image::new(30, 50);
    img.index(0, 50).unwrap();
}

#[test]
fn pixel_setter_and_getters() {
    let mut img = Image::new(100, 200);
    img.set(2, 3, true).unwrap();
    assert!(img.get(2, 3).unwrap());
    assert_eq!(1, img.data.iter().filter(|&&x| x).count());
}

#[test]
fn flip_bits() {
    let (width, height) = (100, 200);
    let mut img = Image::new(width, height);
    img.set(2, 3, true).expect("Indexing error");
    for i in 0..width {
        for j in 0..height {
            img.flip(i, j).unwrap();
        }
    }
    let mut expected = Image::new_with(width, height, true);
    expected.set(2, 3, false).expect("Indexing error");
    assert_eq!(expected.data, img.data);
}
