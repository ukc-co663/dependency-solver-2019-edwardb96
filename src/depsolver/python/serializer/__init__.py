from json import dump
from sys import stdout

def serialize_solution(commands):
    dump(list(map(str, commands)), stdout)
    stdout.write('\n')
