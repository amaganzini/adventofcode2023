import sys

numbers = [
        'zero',
        'one',
        'two',
        'three',
        'four',
        'five',
        'six',
        'seven',
        'eight',
        'nine'
        ]

numbers_reversed = [x[::-1] for x in numbers]


#finds the first number in a line, either an int or string
def get_first_int(line, reverse=False):

    for i, char in enumerate(line):

        if char.isnumeric():
            print('Found %s number as int: %d'
                    % ('last ' if reverse else 'first', int(char)))

            return int(char)

        #i_n is the index of the string list corresponding to the string's val
        for i_n, numstr in enumerate(numbers if not reverse else numbers_reversed):
            try:
                if line[i:i+len(numstr)] == numstr:
                    print('Found %s number as str: %d'
                            % ('last ' if reverse else 'first', i_n))
                    return i_n

            except Exception as e:
                print(e)
                continue

    raise Exception('Number not found')


#find the first and last numbers in a line
#10 * first number in line + first number in reversed line
def get_line_calibration(line):

    line_cal = 10 * get_first_int(line) + get_first_int(line[::-1], reverse=True)
    print('\ttotal line val: %d' % line_cal)
    return line_cal


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
