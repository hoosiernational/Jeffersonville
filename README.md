# Jeffersonville

Simple lightweight Olson-free library for parsing timestamps according to timezone, and vice versa.

All interaction is done through the public methods of two well-documented structs and one enum of timezones.

Note: Conversion from timezones into local timestamps has O(1) complexity due to the hidden ModularTimestamp class, which modulates Gregorian time efficiently. See the source code for more details. 
