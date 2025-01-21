def get_raw(file_path):
    """
    Reads the entire file from the given path and returns the content as a string.

    :param file_path: Path to the file to be read.
    :return: Content of the file as a string.
    """
    with open(file_path, 'r') as file:
        content = file.read()
    return content

terminal_codes = {
    '0': '\033[0m',
    '***': '\033[1m',
    '**': '\033[2m',
    '*': '\033[3m',
    '_': '\033[4m',
    'blink': '\033[5m',
    'switch': '\033[7m',
    'hidden': '\033[8m',
    'black': '\033[30m',
    'red': '\033[31m',
    'green': '\033[32m',
    'yellow': '\033[33m',
    'blue': '\033[34m',
    'magenta': '\033[35m',
    'cyan': '\033[36m',
    'white': '\033[37m',
    'black+': '\033[90m',
    'red+': '\033[91m',
    'green+': '\033[92m',
    'yellow+': '\033[93m',
    'blue+': '\033[94m',
    'magenta+': '\033[95m',
    'cyan+': '\033[96m',
    'white+': '\033[97m',
}

def c(text='TEXT', option='***'):
    global terminal_codes
    return terminal_codes[option] + text + terminal_codes['0']