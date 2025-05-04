import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import statistics

def fn_get_statistics(data, case):
    if case == 0:
        print('Normal operation [Nice Level set to max]')
    elif case == 1:
        print('Normal operation [Nice Level not set]')
    elif case == 2:
        print('High CPU load')
    elif case == 3:
        print('250_000 Normal operation [Nice Level set to max]')
    elif case == 4:
        print('250_000 Normal operation [Nice Level not set]')

    print(f'Mean: {statistics.mean(data)}')
    print(f'Min: {min(data)}')
    print(f'Max: {max(data)}')


def fn_display_50_000_nice():
    data = pd.read_csv('50_000_nice.csv',
                       sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    # data = pd.read_csv('asd.csv',
    #                   sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    ms_data = []
    i = 0
    s = data['Difference in NANOSECONDS']
    for d in s:
        d = int(d / 1000.0)
        # print(d)
        ms_data.append(d)
    print(s)

    fn_get_statistics(ms_data, 0)

    ax = plt.gca()
    ax.set_xlim([0, 500])
    ax.set_ylim([0, 28001])
    plt.hist(ms_data, bins=5000, alpha=0.5, histtype='bar',
             edgecolor='black', color='red')
    plt.ylabel('Frequency [1/50000]')
    plt.xlabel('Difference in Microseconds')
    plt.title('50.000 Measurements')
    plt.xticks(np.arange(0, 1100, step=10), rotation=90)
    plt.ticklabel_format(axis='both', style='plain')
    #ax_x = plt.axes([0, 5001, 0, 50000])
    plt.axis('tight')

    plt.show()

def fn_display_50_000_no_nice():
    data = pd.read_csv('50_000_no_nice.csv',
                       sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    # data = pd.read_csv('asd.csv',
    #                   sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    ms_data = []
    i = 0
    s = data['Difference in NANOSECONDS']
    for d in s:
        d = int(d / 1000.0)
        # print(d)
        ms_data.append(d)
    print(s)

    fn_get_statistics(ms_data, 1)

    ax = plt.gca()
    ax.set_xlim([0, 500])
    ax.set_ylim([0, 28001])
    plt.hist(ms_data, bins=5000, alpha=0.5, histtype='bar',
             edgecolor='black', color='red')
    plt.ylabel('Frequency [1/50000]')
    plt.xlabel('Difference in Microseconds')
    plt.title('50.000 Measurements (no_nice)')
    plt.xticks(np.arange(0, 1100, step=10), rotation=90)
    plt.ticklabel_format(axis='both', style='plain')
    #ax_x = plt.axes([0, 5001, 0, 50000])
    plt.axis('tight')

    plt.show()

def fn_display_50_000_high_cpu_load():
    data = pd.read_csv('50_000_high_cpu_load.csv',
                       sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    # data = pd.read_csv('asd.csv',
    #                   sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    ms_data = []
    i = 0
    s = data['Difference in NANOSECONDS']
    for d in s:
        d = int(d / 1000.0)
        # print(d)
        ms_data.append(d)
    print(s)

    fn_get_statistics(ms_data, 2)

    ax = plt.gca()
    ax.set_xlim([0, 500])
    ax.set_ylim([0, 28001])
    plt.hist(ms_data, bins=2500, alpha=0.5, histtype='bar',
             edgecolor='black', color='red')
    plt.ylabel('Frequency [1/50000]')
    plt.xlabel('Difference in Microseconds')
    plt.title('50.000 Measurements (no_nice)')
    plt.xticks(np.arange(0, 5000, step=500), rotation=90)
    plt.ticklabel_format(axis='both', style='plain')
    #ax_x = plt.axes([0, 5001, 0, 50000])
    plt.axis('tight')

    plt.show()


def fn_display_250_000_nice():
    data = pd.read_csv('250_000_nice.csv',
                       sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    ms_data = []
    i = 0
    s = data['Difference in NANOSECONDS']
    for d in s:
        d = int(d / 1000.0)
        ms_data.append(d)
    print(s)

    fn_get_statistics(ms_data, 3)

    ax = plt.gca()
    ax.set_xlim([0, 500])
    ax.set_ylim([0, 28001])
    plt.hist(ms_data, bins=5000, alpha=0.5, histtype='bar',
             edgecolor='black', color='red')
    plt.ylabel('Frequency [1/50000]')
    plt.xlabel('Difference in Microseconds')
    plt.title('250.000 Measurements (max nice level)')
    plt.xticks(np.arange(0, 500, step=10), rotation=90)
    plt.ticklabel_format(axis='both', style='plain')
    #ax_x = plt.axes([0, 5001, 0, 50000])
    plt.axis('tight')

    plt.show()


def fn_display_250_000_not_nice():
    data = pd.read_csv('250_000_no_nice.csv',
                       sep=',', header=0, usecols=['Difference in NANOSECONDS'])

    ms_data = []
    i = 0
    s = data['Difference in NANOSECONDS']
    for d in s:
        d = int(d / 1000.0)
        ms_data.append(d)
    print(s)

    fn_get_statistics(ms_data, 4)

    ax = plt.gca()
    ax.set_xlim([0, 500])
    ax.set_ylim([0, 28001])
    plt.hist(ms_data, bins=5000, alpha=0.5, histtype='bar',
             edgecolor='black', color='red')
    plt.ylabel('Frequency [1/50000]')
    plt.xlabel('Difference in Microseconds')
    plt.title('250.000 Measurements (no_nice)')
    plt.xticks(np.arange(0, 500, step=10), rotation=90)
    plt.ticklabel_format(axis='both', style='plain')
    #ax_x = plt.axes([0, 5001, 0, 50000])
    plt.axis('tight')

    plt.show()

if __name__ == '__main__':
    fn_display_50_000_nice()
    fn_display_50_000_no_nice()
    fn_display_50_000_high_cpu_load()
    fn_display_250_000_nice()
    fn_display_250_000_not_nice()

