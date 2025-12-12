d,r=50,0
for l in [*open("1.txt")]:d=(d+int(l[1:])*(-1 if'L'in l else 1))%100;r+=d<1
print(r)
