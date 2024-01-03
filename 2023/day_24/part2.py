from sympy import symbols, Eq, linsolve, pprint

#Define symbols
x, y, z, vx, vy, vz = symbols('x y z vx vy vz')

x0, y0, z0, vx0, vy0, vz0 = 230027994633462, 224850233272831, 164872865225455, 103, -57, 285
x1, y1, z1, vx1, vy1, vz1 = 213762157019377, 204038908928791, 198113097692531, 184, -110, 174
x2, y2, z2, vx2, vy2, vz2 = 236440979253526, 104012423941037, 223798957622735, 15, 694, -277

#Simplifying the equations and eliminating nonlinearities produces the follow matrix equation
equations = [
    Eq((vy0-vy1)*x+(vx1-vx0)*y+(y1-y0)*vx+(x0-x1)*vy, x0*vy0-y0*vx0-x1*vy1+y1*vx1),
    Eq((vy0-vy2)*x+(vx2-vx0)*y+(y2-y0)*vx+(x0-x2)*vy, x0*vy0-y0*vx0-x2*vy2+y2*vx2),
    Eq((vz0-vz1)*x+(vx1-vx0)*z+(z1-z0)*vx+(x0-x1)*vz, x0*vz0-z0*vx0-x1*vz1+z1*vx1),
    Eq((vz0-vz2)*x+(vx2-vx0)*z+(z2-z0)*vx+(x0-x2)*vz, x0*vz0-z0*vx0-x2*vz2+z2*vx2),
    Eq((vz0-vz1)*y+(vy1-vy0)*z+(z1-z0)*vy+(y0-y1)*vz, y0*vz0-z0*vy0-y1*vz1+z1*vy1),
    Eq((vz0-vz2)*y+(vy2-vy0)*z+(z2-z0)*vy+(y0-y2)*vz, y0*vz0-z0*vy0-y2*vz2+z2*vy2),
]

solutions = linsolve(equations, x,y,z,vx,vy,vz)

pprint(solutions)

for solution in solutions:
    x, y, z, vx, vy, vz = solution
    print(x+y+z)
