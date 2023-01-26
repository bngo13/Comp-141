import argparse

parser = argparse.ArgumentParser()

parser.add_argument('--input')      # option that takes a value
parser.add_argument('--output')  # on/off flag

args = parser.parse_args()

inputs = args.input
output = args.output

for char in input:
    print(char)
