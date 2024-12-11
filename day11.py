import csv
import numpy as np
import matplotlib.pyplot as plt

single = np.genfromtxt("day11-single.csv", delimiter=',')
parallel = np.genfromtxt("day11-parallel.csv", delimiter=',')
parallel_fast = np.genfromtxt("day11-parallel-fast.csv", delimiter=',')

parallel_fast= np.hstack([parallel_fast, np.expand_dims(np.log10(parallel_fast[:,2]),1)])

fit = np.polynomial.polynomial.polyfit(parallel_fast[25:,0], parallel_fast[25:, 3], 1)
x_extend = np.arange(parallel_fast[-1,0],76)
y_extend = 10.0**np.polynomial.polynomial.polyval(x_extend, fit)
factor = 10.0 ** fit[1]
print(factor)
print("Blink {:.0f}: Secs:{:.0f}, Minutes:{:.0f}, Hours:{:.1f}, Days:{:.1f}".format(x_extend[-1], y_extend[-1], y_extend[-1]/60, y_extend[-1]/3600, y_extend[-1]/(3600*24)))
cpu_score = 12119
ryzen_score = 66372
cpu_scale = ryzen_score / cpu_score
time = y_extend[-1] / cpu_scale

print("CPU scale {:.1f}, Hours: {:.1f}".format(cpu_scale, time/3600))
fig, ax = plt.subplots()
ax.semilogy(single[:,0], single[:,2],)
ax.semilogy(parallel[:,0], parallel[:,2])
ax.semilogy(parallel_fast[:,0], parallel_fast[:,2])
ax.semilogy(x_extend, y_extend, 'g--')
ax.semilogy(75, time, 'rx')
ax.semilogy(75, y_extend[-1], 'kx')
ax.semilogy(parallel_fast[-1,0], parallel_fast[-1,2], 'kx')

ax.text(65,500, "Ryzen 9 9950X\n{:.1f} Hours".format(time/3600),  color=[1,0,0])
ax.text(55,100000, "Core i5 10400\n{:.1f} Hours".format(y_extend[-1]/3600),  color=[0,0,0])
ax.text(45,1000, "{:.0f} blinks\n{:.1f} Mins".format(parallel_fast[-1,0], parallel_fast[-1,2]/60),  color=[0,0,0])
ax.text(30,0.0001, "Exponential factor: {:.3f} ".format(factor),  color=[0,0,0])
ax.grid()
ax.legend(["Single Core", "Parallel", "Parallel Fast"])
ax.set_xlabel("Blinks")
ax.set_ylabel("Seconds")
plt.savefig("day11.png", bbox_inches='tight')