const a = [5]i32{1,2,3,4,5};

//implicit infer from the return type 
const b:[5]i32 = .{1,2,3,4,5};

const c = [_]i32{1,2,3,4,5};

