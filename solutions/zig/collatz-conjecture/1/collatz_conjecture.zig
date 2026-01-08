// Please implement the `ComputationError.IllegalArgument` error.
pub const ComputationError = error{IllegalArgument};

pub fn steps(number: usize) anyerror!usize {
    if (number == 0) return ComputationError.IllegalArgument;

    var x = number;
    var step_cnt: usize = 0;
    while (x != 1) : (step_cnt += 1) {
        x = if (x & 1 == 0) x / 2 else x * 3 + 1;
    }

    return step_cnt;
}
