import os
import sys
import glob
from ctypes import *
import time

#get path to this file.
path = os.path.dirname(os.path.realpath(__file__))

#determine dynamic library extension based on platform
ext = ""
if sys.platform == "linux" or sys.platform == "linux2":
	ext = ".so"
elif sys.platform == "darwin":
	ext = ".dylib"
elif sys.platform == "win32":
	ext = ".dll"
else:
	print("Error: Unrecognized Platform.")
	sys.exit(-1)

#find rust library
f = glob.glob("%s/target/*%s"%(path,ext))
if not f:
	print("Error: Failed to find rust library. You may need to run `cargo build` first.")
	sys.exit(-1)
print(f[0])

#load the library
nxtlib = cdll.LoadLibrary(f[0])

port_a = c_uint8.in_dll(nxtlib, "PORT_A")

nxtlib.get_nxt.restype = c_void_p
nxt = nxtlib.get_nxt()
if not nxt:
	print("Error: Failed to get nxt.")
	sys.exit(-1)


nxtlib.play_tone.restype = c_int
nxtlib.play_tone.argtypes = [c_void_p, c_uint16, c_uint16]
err = nxtlib.play_tone(nxt, 440, 500)
if not err == 1:
	print("Error: Failed to play tone, error code: %d"%(err))


nxtlib.run_motor.restype = c_int
nxtlib.run_motor.argtypes = [c_void_p, c_uint8, c_uint8]
nxtlib.stop_motor.restype = c_int
nxtlib.stop_motor.argtypes = [c_void_p, c_uint8]


err = nxtlib.run_motor(nxt, port_a, 75)
if not err == 1:
	print("Error: Failed to start motor, error code: %d"%(err))
	sys.exit(-1)

time.sleep(0.5)

err = nxtlib.stop_motor(nxt, port_a)
if not err == 1:
	print("Error: Failed to stop motor, error code: %d"%(err))
	sys.exit(-1)


#nxtlib.free_nxt.restype = c_void_p
nxtlib.free_nxt.argtypes = [c_void_p]
nxtlib.free_nxt(nxt)

