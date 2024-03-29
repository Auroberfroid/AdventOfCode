import math
import matplotlib.pyplot as plt

bef_pts = [[74, 72], [74, 73], [72, 73], [73, 74], [73, 72], [72, 72], [74, 74], [72, 74], [73, 71], [73, 73], [72, 71], [74, 71], [71, 72], [71, 71], [71, 73]]
origin = [72, 72]
refvec = [1, 0]


def clockwiseangle_and_distance(point):
    # Vector between point and the origin: v = p - o
    vector = [point[0]-origin[0], point[1]-origin[1]]
    # Length of vector: ||v||
    lenvector = math.hypot(vector[0], vector[1])
    # If length is zero there is no angle
    if lenvector == 0:
        return -math.pi, 0
    # Normalize vector: v/||v||
    normalized = [vector[0]/lenvector, vector[1]/lenvector]
    dotprod  = normalized[0]*refvec[0] + normalized[1]*refvec[1]     # x1*x2 + y1*y2
    diffprod = refvec[1]*normalized[0] - refvec[0]*normalized[1]     # x1*y2 - y1*x2
    angle = math.atan2(diffprod, dotprod)
    # Negative angles represent counter-clockwise angles so we need to subtract them 
    # from 2*pi (360 degrees)
    if angle < 0:
        return 2*math.pi+angle, lenvector
    # I return first the angle because that's the primary sorting criterium
    # but if two vectors have the same angle then the shorter distance should come first.
    return angle, lenvector

bef_x_values, bef_y_values = zip(*bef_pts)
bef_fig, bef_axe = plt.subplots()
bef_axe.plot(bef_x_values, bef_y_values)
bef_axe.set_title('BEFORE')

aft_pts = sorted(bef_pts, key=clockwiseangle_and_distance)

aft_x_values, aft_y_values = zip(*aft_pts)
aft_fig, aft_axe = plt.subplots()
aft_axe.plot(aft_x_values, aft_y_values)
aft_axe.set_title('AFTER')

plt.show()