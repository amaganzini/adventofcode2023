import sys


def get_first_int(line):

    for x in line:
        if x.isnumeric():
            return int(x)

    raise Exception('Number not found')


def get_line_calibration(line):

    return 10 * get_first_int(line) + get_first_int(line[::-1])


def cli_main(filename):

    calibration_val = 0

    print('Opening %s to read calibration value...' % filename)
    with open(filename, 'r') as f:

        for line in f:
            calibration_val += get_line_calibration(line)

    print('Calibration value: %d' % calibration_val)


if __name__ == '__main__':

    filename = 'input'

    if len(sys.argv) > 1:
        filename = sys.argv[1]

    cli_main(filename)
