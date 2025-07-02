set datafile separator ","
set terminal pngcairo size 1000,700 enhanced font "Arial,12"
set output "burned_vs_density.png"

set title "Procent spalonych drzew w zależności od gęstości lasu"
set xlabel "Gęstość zalesienia (%)"
set ylabel "Średni procent spalonych drzew (%)"
set grid
set key off

plot "results.csv" using 1:2 with linespoints lt 1 lw 2 pt 7 ps 1.5 lc rgb "blue"
