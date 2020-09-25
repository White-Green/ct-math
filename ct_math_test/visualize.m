table = readtable("result.csv");
add_dynamic = table(table.type==0,2:3);
add_static = table(table.type==2,2:3);
[agroup,asizes]=findgroups(add_dynamic.size,add_static.size);
admedians=splitapply(@median,add_dynamic.time,agroup);
asmedians=splitapply(@median,add_static.time,agroup);
mul_dynamic = table(table.type==1,2:3);
mul_static = table(table.type==3,2:3);
[mgroup,msizes]=findgroups(mul_dynamic.size,mul_static.size);
mdmedians=splitapply(@median,mul_dynamic.time,mgroup);
msmedians=splitapply(@median,mul_static.time,mgroup);
fadd = figure("Name","Add","NumberTitle","off");
fmul = figure("Name","Mul","NumberTitle","off");
figure(fadd);
semilogx(asizes,asmedians./admedians.*100,"-or");
axis([0 max(asizes) 0 100])
title("動的サイズ行列に対する静的サイズ行列の処理時間割合 和")
xlabel("行列サイズ")
ylabel("処理時間割合[%]")
figure(fmul);
semilogx(msizes,msmedians./mdmedians.*100,"-or");
axis([0 max(asizes) 0 100])
title("動的サイズ行列に対する静的サイズ行列の処理時間割合 積")
xlabel("行列サイズ")
ylabel("処理時間割合[%]")