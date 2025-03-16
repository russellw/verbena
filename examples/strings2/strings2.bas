10 REM STRING OPERATIONS TEST PROGRAM IN BASIC
20 REM This program demonstrates various string operations
30 PRINT "STRING OPERATIONS TEST PROGRAM"
40 PRINT "==============================="
50 PRINT
60 REM Initialize test string
70 A$ = "HELLO WORLD"
80 PRINT "Test string: "; A$
90 PRINT
100 REM String length demonstration
110 PRINT "LEN function test:"
120 PRINT "Length of '"; A$; "' is"; LEN(A$)
130 PRINT
140 REM LEFT$ function demonstration
150 PRINT "LEFT$ function test:"
160 PRINT "LEFT$(A$,5) = "; LEFT$(A$,5)
170 PRINT
180 REM RIGHT$ function demonstration
190 PRINT "RIGHT$ function test:"
200 PRINT "RIGHT$(A$,5) = "; RIGHT$(A$,5)
210 PRINT
220 REM MID$ function demonstration
230 PRINT "MID$ function test:"
240 PRINT "MID$(A$,7,5) = "; MID$(A$,7,5)
250 PRINT
260 REM String concatenation
270 PRINT "String concatenation test:"
280 B$ = "BASIC "
290 C$ = "PROGRAMMING"
300 PRINT B$; "concatenated with "; C$; " = "; B$ + C$
310 PRINT
320 REM ASC and CHR$ functions
330 PRINT "ASC and CHR$ functions test:"
340 PRINT "ASC('A') = "; ASC("A")
350 PRINT "CHR$(65) = "; CHR$(65)
360 PRINT
370 REM String comparison
380 PRINT "String comparison test:"
390 D$ = "HELLO"
400 E$ = "HELLO"
410 F$ = "WORLD"
420 PRINT "Comparing '"; D$; "' and '"; E$; "': ";
430 IF D$ = E$ THEN PRINT "Strings are equal"
440 PRINT "Comparing '"; D$; "' and '"; F$; "': ";
450 IF D$ <> F$ THEN PRINT "Strings are not equal"
460 PRINT
470 REM String search
480 PRINT "INSTR function test:"
490 G$ = "PROGRAMMING IN BASIC"
500 PRINT "INSTR(G$,'BASIC') = "; INSTR(G$,"BASIC")
510 PRINT
520 REM String conversion
530 PRINT "String conversion test:"
540 H = 123.45
550 PRINT "STR$("; H; ") = "; STR$(H)
560 I$ = "456"
570 PRINT "VAL('"; I$; "') = "; VAL(I$)
580 PRINT
590 REM String case conversion (where available in some BASIC variants)
600 PRINT "String case conversion test:"
610 J$ = "Mixed Case String"
620 PRINT "Original: "; J$
630 PRINT "UCASE$(J$) = "; UCASE$(J$)
640 PRINT "LCASE$(J$) = "; LCASE$(J$)
650 PRINT
660 PRINT "End of string operations test program"
670 END
