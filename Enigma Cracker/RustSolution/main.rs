mod enigma;
use enigma::Enigma;
use enigma::rotor::Rotor;
use enigma::plugboard::Plugboard;
use std::{io, thread};
use std::sync::{Arc, Mutex};

const ORIG: &[u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'];

const UFW_B_MAP: &[u8] = &[b'w', b'z', b'u', b'L', b'?', b's', b'"', b'8', b'7', b'M', b'J', b'F', b'd', b'c', b'/', b':', b'#', b'$', b'>', b'q', b'o', b'|', b'@', b',', b'k', b'B', b'j', b' ', b'5', b'R', b'2', b'G', b'0', b'}', b'T', b'1', b'{', b'p', b'N', b'Q', b'K', b'b', b'v', b'O', b')', b'a', b'E', b'3', b'9', b'C', b'H', b'+', b'D', b't', b'^', b'y', b'Z', b'_', b';', b'`', b'!', b'U', b'Y', b'6', b'g', b'h', b'&', b'%', b'r', b'=', b'I', b'.', b'P', b'n', b'<', b'*', b'e', b'f', b'W', b'-', b'(', b'i', b'4', b'm', b']', b'~', b'[', b'S', b'V', b'X', b'A', b'l', b'x', b'\\'];

const R1_MAP: &[u8] = &[b':', b'-', b'l', b'e', b'r', b'_', b'$', b't', b'J', b'B', b';', b'w', b'F', b'P', b'h', b'.', b'U', b' ', b'G', b'9', b')', b'2', b'4', b'f', b'=', b'j', b'v', b'>', b'+', b'0', b'x', b'Q', b'@', b'W', b'o', b'(', b'L', b'8', b'`', b'D', b'Y', b'k', b'g', b'I', b'E', b'V', b'Z', b'<', b'%', b'7', b'"', b'm', b'/', b'y', b'^', b',', b'#', b'C', b'O', b'b', b'}', b'3', b'?', b'!', b'\\', b'd', b']', b'*', b'H', b'|', b'i', b'z', b'&', b'n', b'5', b'A', b'T', b'6', b'c', b'[', b'p', b'~', b'K', b'X', b'{', b'a', b'u', b'N', b'1', b'R', b'q', b'M', b'S', b's'];

const R2_MAP: &[u8] = &[b'{', b'/', b'1', b'^', b'V', b':', b'_', b'a', b'9', b'e', b'd', b'b', b'2', b'0', b'y', b'&', b'[', b';', b'w', b'k', b'j', b'P', b'*', b'M', b'D', b'Q', b'u', b'~', b'B', b'%', b'R', b'q', b'<', b'C', b'v', b'|', b'5', b'W', b't', b'p', b')', b'Y', b'-', b'4', b'i', b'.', b'H', b',', b'K', b'A', b'S', b'"', b'x', b'6', b'=', b'}', b'3', b'E', b'7', b'N', b'!', b'F', b'm', b'T', b'J', b'?', b'#', b'+', b'c', b'r', b'z', b'U', b'L', b'Z', b']', b'$', b'h', b'n', b'\\', b'g', b'o', b'>', b'@', b'l', b's', b' ', b'f', b'I', b'8', b'(', b'O', b'G', b'X', b'`'];

const R3_MAP: &[u8] = &[b'T', b'y', b'l', b':', b'p', b'2', b'>', b'E', b'o', b'Q', b'+', b'f', b' ', b'!', b'(', b'M', b'L', b's', b'J', b'&', b'P', b'r', b'U', b'$', b'.', b'Y', b'|', b']', b'C', b';', b'^', b'W', b'9', b'?', b'{', b'D', b'n', b'%', b'@', b'V', b'I', b'7', b'B', b'N', b'u', b'6', b'~', b'"', b'5', b'q', b'c', b'K', b'0', b'4', b'`', b'\\', b'}', b'-', b'e', b'w', b'i', b'h', b'g', b'm', b',', b'b', b'k', b'3', b'X', b'=', b'[', b'j', b'R', b'O', b'_', b'v', b'H', b'd', b'<', b'x', b'#', b'A', b'*', b'z', b'/', b')', b't', b'S', b'a', b'G', b'F', b'1', b'8', b'Z'];

static KNOWN_PLAINTEXT:&str = "Wetterbericht: // Datum: 15. Oktober 1940 // Einsatzort: Sonnenberg // Meldung! Meldung! Hier spricht der Wetterdienst fur den 15. Oktober 1940 im Einsatzgebiet Sonnenberg. // Die Wetterlage fur morgen wird voraussichtlich bedeckt sein, mit starkem Wind aus Osten. Die Temperaturen erreichen ein Maximum von rund 12C, was kuhler als gestern ist. // Es besteht eine hohe Wahrscheinlichkeit fur Niederschlage, mit einer Moglichkeit von Regen wahrend des Nachmittags. Alle Einheiten werden darauf hingewiesen, dass entsprechende Kleidung und Ausrustung fur die geplanten Operationen mitgefuhrt werden mussen. // Sicherheitshinweis: Bei Anderungen der Wetterlage sind die Kommandanten verantwortlich, die notwendigen Massnahmen zum Schutz der Truppen und Ausrustung zu ergreifen. // Das war der Wetterbericht. Bleiben Sie wachsam und passen Sie sich den Wetterbedingungen an. // Weitere Befehle oder Informationen konnen angefordert werden. Das war der Wetterdienst. // Heil Hitler!";
    
const KNOWN_CIPHERTEXT:&str = "@pEbOB!_8/ep%A\":70@Um-3m)Mn}uSI,9t49bt7\"8idYU-z<m_mBY6}bwQ7x#;%gmTk3h.)[u&!M5PPaV!;qe/ieT/.|dK>fe7-R?3rDH.BA@D3Z}S]j?X?e-+[wA\\G+(O+7sc?WpTJxWj #.1=lz+~W9IQDl8Xaj4+926Gu,#L[HbAKI9a={?BRXYy&Vt.Q,>:_v-vx*g8n>nvV.5-Y~.+i&SYQp*:2W]8tn+r,H-qR^r3RrY]MO\\!jTYbx0x64yWkp<r?7L<6)Nn\\YKvCl3*/mO_G6/:5v(l9h97mWj6!dkfAp!\\yzx;BRkTfk=*AzY,$f8*rXtXr5aZ_K3B/7V$3J<j2\">,UqM`5kX*G[v,69}%<aUFfA)$hkd*eA=bL@R[mJjq(QZF #[2Ll0<fdFQ5037xQJL${p+vs|Yd,|FnQL@b@8%QK-j?[q7ko8\\h>lmedS^1$@(1%O%Q8p}+d*0P:xM]KX} G\\;fRF9[5(}z}2U/ZCEOtTyGdW@vvZ=^tb71u(p[64/fv5/VD#y}PSsZF7x7z%9vcglF#n2d4wdp<Jn\\QGqU+o=GL56H:t*G&PJrFXa]B2}#wgm]82R\"Qw-dVn%EOA5-AKs69n3+]Sb(m$]\\sYf,qZ8;g&%fO$lF=QnS`9v0-?d8opR>P\"#l2<`Mf1^C8V3A))gw+.L:Q}Dc9\"o^Q\"Ng>rais\\jz<S1:denJ{!|zNIdZ?[NS~YOh[}np0yfdxbE8~#Q>l% &kwfO]@2u%DczR!nT&Rt%>K6sCUEUi&/uXbo7S7uDqgkLpb?D{fOTq`+6RA[wA$Qdm\\0O&zpR@47J[TV:s(~\"Wm^V_L~,$o%L=&`t3,o74sf|!\"oXM$^g50lU`~zDNxss,U3I),rX.$^YL}h%I4Egx@,f[pkgBUX=|qBuYtx$mKFhb4j%%CRwhF}8up(X(>-GvK/x-kIqYaJtYqG/~z5Rapr5W9kVEI?gQ(Q>DjUW 87";

const CIPHERTEXT_FLAG:&str = "ro\"oqSA=\"I.Vwr@bw2?d*;_~M8f+|Q>pUB49X1A%jiHIM%p<b\"e%Y68P|c ]|z[m%W=78)Nxu5I*>D*{3[t;uU<(#=|[Zs!Zrh(Nc9psZ!nX>Hs/.|i6FBWSpIF\"?wdV O++8_`I:$T+X&0%?aRj\\w#wILQD?#za2H]ti6G#=\"|ssJoWH4*SG-i1HIz7#^.F#$`qMCPx5dis7l|/T+{?ythu41>Rt7x=lwCh8djXi#-8^PWP?7c=d{@0ePMW_.xrl[9p<e)0XO6yqD\\+)<1ug&n,O_<}2P]E.ri7]qml?OP<PX;}5D$<7Uwg9_Ut8?L.b6 _r,A.dwCQu!^K|iP=X;Qd@NQ*9=3501><$0ZXK)<9+7AvtFqg;ZsU!zezRfswG`lG45-#i_i8e:hwr9vMK~!b!2xn6dB]9+KVS|G:1q~@?N^k#%MZ$p@qFsPz72vJ-hITYXH7kyj%}]:2x810J{B_-C ez} GV;fY [2`s3s12Z6e5Skh&a&5#vM%e=]XYF1Eg}IC[[$<|-WHNT]~u|Z(Mj7Low|!$kE8k2ScXdV@p>%[:$]=9H@[\"yl7;RERiZ=i3>;86~xl|@)K6ws-o?-SQ]u[HMzI,MtTr>c:\\W:oiow{j.dQZ&VknT6BPV[YAvQO\"vC\\V V,jkP65Llu8`WRD(C0l0pojM}b]]s0(69L/oU pefD\"h0LyT>Lu-r><d7%vSE88@`=Y!=g?OC?8h/\"1*_eQH*|E>u$YSgIJ_~Cb)uc1`+1!nqQbc7S[\"^*fE2t&9DVHgR>So3R2;^.e]6\"f%JqfXR6Fh`3**%~+V2+id6`%8&66U]NP\\&>Y(c0L#?)tsysBEii<o\"8Aft)b@@b5S!D?1\"u.n!FRaxLm=2Fw9=\\60Me^+m_)lE&1w{HmzF}=OB&Lh[_sUi!CQ,{15`e]=wqaK4wh>dya])GyYeAsQA2)VEQ<j4Va{\"2A$.nvY~D+]Kl;;5;fKve:WjateB3-Yr/yJgJi3)RF1 N#&wT1a,*\"l&2p\\s 5F ?G|]%Wiw~:NeK:8shKclcx\"C>y;.k44dg<G9_h RAv2B]4XHD<rX#)no&5+$]$ K1A^VfuCF4QvW;NS.rWq<\\I,O8]L2]`~\\E$&$ZNM%|`QbC/}xI&j*m/S2rMlK/,\\lqVm]M9:O0[j#HC/AR*A6#8qc%lza]_e&3EeH8\\eV@fPVT1UV6{\\xKs;/_,*H]$G+s:KQRDW^Ss1r!NFy6K9mi~ NSM&^B!A/Q 0<2<&tP|b .oq=|+\\^]bh5.k<cPTq4cj5;vk4QIT:m=]6s{Z(F%|pA~jBR\"]/(bqc}Gx.vmVk)S&`#f@H~ZM:~T#E=POlt&aly>ObJ^X2rVqrxI%;E|SiQhjZ59^QQK69s{1)J.E ,A+BqGHW43M1-2\\/5u3>bY;AK?6ST@-hHkxHyBH;S,-ZFk-/I7g*}!4h#OwtO49i@#s*pKf$N6NQU\\G>K0cdV{pH&/ZEY~fa5;xbrT!Ki\"=1fr-|@$f+}~0Ot1`AId!L#PsUpFmiH wr}$0ms#*d=5k=F#/(>FC?YMT5h_6a>Rs[tdd]Q%[c$Z>`7,^GAFOSanr>NjGq&ih}vo7l}qmW&g?*h]>HL7%i(sm%A#Ho4(>IG?%z),xeFm.:>GP(?>lSws64zh3Xw1,[Uitj+;q,+EmCH+3Y)XBKvAD1\\YKH+=Fv[D\\/SGPTug5=kQ/xbKg\\.*BwM.k[dSj]$:>} vAoMgU6fv(.<CuL3&N,<<hHQ+pni6D=H|9B_{r:VN,|+Bl}[Ze=R^@Sv?D2>:>m>:<YD5d#ZBKm7p<DOMQs;Cou=HGa\\S&/~LfFO}F^f)$E8|C@@H#@4km2P1\\aOl://2-yK>*xg[:AJ$65F_w>c{9_ZA@;T]E;t1,lh^|{X pNyDECbK/Egm,c6/KUT70DIkT`>cNA^0sR\"R5l$Cpi1g`9+LOke\\dw\"#X8N#LVMW0h:iC4LLaGO0i7X\\82`{@UqU\"tfm@/eh dtM8C\"x%jAzvK~.>{F\"1R#j<\\=9S/<^sb*?A]kU`ILPja~;bgoq*[2b0{Mi/G{TLM#GXqe:&#W.T:]5y=x.~<~Jqo`J#idZaP!Q%a}HC`U\\$[{oF)|Fy{9YE(d#9\\C\\jPpS;}Sl`C8MQ8z: }pIiA*Y1l,KGp2v]ZD~6(B1[R.Qdc>@dLs0q:uJe3OMF5lJ9\"b\\Dy{6P9z!@J_\"oSNg4:Ve,z(2v{N`kJ2YY<xOydC9Q5,:ZxS34uxa[~-sX!ydm\"/vq;bGX\\t]gG,P,U4lYl!VZn]\\al02OdQ%0\\sbd>-IP=icCDQ4@ast<83\\cE,63,%yxW(8,*|$PQi)V|\\*O(hw$tm`[roh6)3eo8vp!6T!R]8k/jeg0lAxz~^w9tsF@I![n$:|*lB7byY 29WL0M[\\x~,jrB^PEa}P~UUC(o!.iN`.V1he3ApEdI8RIV:Nu 0R=9-~So,s4p>xz8wey2C!@-NJEX.jcW~05JTfA[ifr_Hc5uRzG&B^pg!=E-BECO?Ygr,0\"js\"\"*xj*/YTI)IK/<a1_Ag\\B0w9c|F7v9iY9H}bJN~!!EKQ,fXGdA)}|raYvyD?8ROALQ(bE2.$w{^k]YC)i6\"Yb4U%OLOP>,sAI;j56\\K*Z8cBlnfR]sX*gkW49DDQFpsJH)rjxMU,}S8M|\\B|mA~}\"E79+Y X*Jh_xweF$$wy~leZmg_3{xiN-ezU* P`0y/Epadh3m<EsU`mjiI`c~@H=-*3J)&8@6gede3&]k0v6$_TX{[E7sv7N^O#(p%?C^J6kpg&l{kOOy_^3@HUmVPnZj!/Z1]\\46Jj::-H4kYY]H3?1hEpzQ^zQ}o)2WUeU9?DeLMl_.\\rM->mE8CF]],?75\"iHA,%oSklPJq]xiOKN1@0B(#v~H<\"lB]zez4SinwJ]*cAY0]2*n^,lZVMw4zrGsP-Itx5!7U}LMb/=}F(MOR*B\\9[_o\"rop,ZFP=%_#ow!p9B6CD!pOd:ZIFv[ ]l1I}j?K<jT y^7Ba54l<Y~/`7NRB`MrEURzGi&+JYKZZ;z+>QY.5j\"eF]/^^`\\(qk-_Fq@@Y#,/hoStZu{xsy&f/Q[f(212+%W+|\" #8:bdhm9y!pc0O}(.5p~ m,6$}<T5<J1JW$JBra`yUrq%Qf%C7 0#l2,gL>^btV/8TjD|kQ?&j==jp3wHSJZ}RK;!QiCcuSckc_5YB\\K,[uU($;P<<PD;*L2Qw> }G{9t_X/Zx5H@fgYNv]UgIG+O!k@,vgNM`~Ej^O&=G,~_Ds\\w|.I/~$IS`_$RjCPSf63t}|3?BdfbXY{/Bh%7Rn5tb!IYXPI`;z};;z<v~GRJqz%7QPIZG$Rk> o`|z$q6wRMA>L>ODNi>%otg[}\"+CC<2fC^N3Z|$^gm{Gjy5V^4h?B]s}Y.o0!/ch3uk9cu#\\=PBj]Z>h(s8k{G05H2|R_Mo2IhTTw0@@k| _za\\_2Bd-F|j)<5Sx|IdQJ84uR~U`Co{1<B_u4Uq$nUjJCQG=$$ sH\\;AFr3B=Fy`_[?Ft/4D0GDgmA6ig$Vr8,jMonr=;=`(4ej pj]u~.%KgjL@`~nLz/\"n-O/X<K\"^Dht9nYuE{5HF-JlYz$Zl5=t0z,q8fjUyHdd/Cr#JzoR.4$)Ys}nrr7z`uOG|9T4]W9wL#>x}-ZHmX&B]2F65cndcjrqti=8kva0[nbDs!^d&Lr.do>DLSt+vDjbw)LuX9K~sW1_@|]71S{A/nj(g>1@D\\[goY<Alk8A88(!M<qkK#SmJWIVN)%oJRK*}XxQ(pQA\"`a)y.,[FXJZWV;~7^HUhb](T]kO_tO_T4TXuf0.vnNk%GOD,_3r=>uWPON);:sB{hb@t5*cgH]w=54Cag^Q\\!aJ$)6Pyvx1gjX grG9ctNUv2)QHP>-.((E8ZGla]#$.<HH_@Pd^$ma#C:Oi?|RcMY5J{Q7.@AMtSTdHt`+b8JK/S:xp^C)wp6~<a!,2>aSKbG#f]XDu$ye|SUA81o_i#l?i(0w,mC*D!JkGD}T%8tD`yYy&RIZxl0y9xjRN55;>4x4|\\+zPgyYp-92KJMDB2Xq,&M|%T|gC-N|8KwYHq?6bxt;I.xYk8m=c0IsLp>FM{PbA[I\\9Et;n9|_6rF(w~,f PEPJ?rUMn 5>qPZraAlztKP9chpVtQ{R<tzEe#?y]^Atrjb_k^adqP&+e.&^j[\"jDc+]Pt /G8ruut,Ax3ju^^d8$[y50dDH0VgK[R@iO!^J:}9w3#_8Q8yKhdsc42BRifd@eE-aJj,~%86#5W^em\"uu+BHuXfhR3,DA)9(5QG_9[@=K$@5H%B)~Qef^8>g2;t6pOL!&u=gz50j{H NNjaYi1BZ-o?: MD<ql]=JxA9[|g*-P0)smY>J^m{;Pg;mfP`*bdf9{1*E+0V@cWTT%uNj8nZH2_y7Ag@5!=e~R/zu-Zq(zC6:H^~?MN)_5\"0=Kq(uVMm8!@WAF~\"()7.r/+1#(S<zZ.{Y+BNLqH{mrN&]C+ewWo~u]cNW#4)_u1lwhAe2Vbt2JIU[H+Sk4A;pZu.[9Cnor:W!-`rK;y/Wkp^rv+[O/Wic&/4,xG|\";(H;r4!8?&+}SlLY<^NFRwlh<WK&_..&0FBYq^#5jyYI?><`1SZmB{`t@FqxhB9@XL),H=\\Olup;3)2I{rn)~f8~7Lz0b_6HOV|~y|vu~\"1@,2Z)F-#GOXK79~0r|v;YNsgcal7N3`**4e@O:g<X\\=gsem^s)K0%pVGT1~/]TPpgXmGp`vEE]|fd,^tbb[[#816\"\"G8(A+kI;tR[yI09wa@FMI;F@?MC>(/-eSE?g[OPZjG58}tF3tWatERq0 4v\"%.;S,eA<ZAc2]j5>^+I}{TY\\&Dp<N9SCOv2wi=fl2^3!jI:&5FLn&!i-h)#jFzm<\"1%7buNaMhkqI3cSmVw$Qi!%PjZ&h/.uD[7>AgMvW6r0|&HT`zMqs~xwc$j5xugUNJj7\\5qz8U|=KcxG`XSxjzJ4`$O-8Zb`d#V/o)G[qE,\\e14GY=P-${UT_- _7J9#lq:WbYL=5i~-Bg>oMu0yE}\\kJ&j7^V2EnksXC(RnO[t{u`>P@SjL4D0eB]2?6;v8{sN1Pe+l=zem]gn5dY7D4|ZNHusa}db,}q]bAb](]ezvsPaY5%0VQI[8#,]&D<+Rh{M\"xIdbTd@D|pMdW!,!y>j=ZmB_VFhzCe8_s[&(l2W{Zn)gwf-jBo#~RAUwHdk/eQVcb{>hl9Cs>38&wF$SBXfgxu{L.rH8y{MN*,-:#iU[dy!{4Xqs\\d*4,Zaho(N@y]_vV|PB{=O<|XmWf^Toi4KA%3fV2.g&Dcuqu27E0AOm-[V!y3|^#@+~.eD)o%mEJoa]4KT*&N2g.z^n@hKcC`)9*-si_51-Ovq&1$!.qu_/bXW6Y7_1`:K]JUYLQ900/\"l,{6;ulsCC*`hQ<`QLh)K$|%J8O4+s062$ZR6gX2X@O>(ve2QZl*nNmPW.\\-Jv!+sTm/f&D!GVZF4D#4Er,]yif}`5>fqZ&DbIn!&Dwh*zV0[igzNn6_+o:=by]|1z.y(2fro#3CUV){@5)3^UNH2.$Kw[u_~rz2<)\",d7Rlhj]oOSftFy{#;a6&^k\\2vD,it3(@:K$[!Fc#/3sFkGTf.<vyD\\lqzQX*MLaZ/Qs!XL<KWX2m&=NN3_Q9u_X%NXU*8d2I.4}W1\\\":\"I`=<4J{L;R4cZ?l$V<t&c?Ao%,aZ,fY}jFR>/ePA7,G1Q\\WT>d+,|{0:nWp|n\"AM\\jMT7)TCmGsz/DlKxCyOqP>UG(,#N* Gq~\"J\\S{FJs.3~3hVUK.rG]543sL[j\\uZBF5/lR{W,0-M*mk1@DA&$kVa$GMXJ2{$7wwW97{>#kWm<O~nG*#SMeJS\\kQl0S,Z+Fo?X+<aj=FfI{vXYUuvotb7 /(Daqj@ Tl\\kUP6KFg#/?eWG_YAL/</qa`l2#,ar7WGpzGsQTO:+_eDpp[h(k{\\m#BT{R;Yv.+B^{%laSU\\fY!.qhppZGG&/+DPR7dRs\\x3/%I)k=KvwCUQ4P)Lq2cC^-/C34&raWFZ`lES9Fx^h$bjBXqP4e&j[0hKTh<I7\\:a=t9};%}\\B38QoM+_SxXj0g%=Cz sL0}B:E]J}UiPYd52qz=q}X&M}9qFik=6Fz ?Oy;7,I1~xd,WXbq5BRHB.c0[W:VjnW4I:s !@xG_]a|nJ!CNu\\$nNA@c\"/#3-Cp9zz4q=^x*>jDI1ok9MyzXwlcn&DO<ARsh:?ipI?p`(L%g%5(Zxm\"d\"8.,/31Kgv] 814M]1RC!c\\$k\\7(D.5KP%US q\\DTazbO6BRAg4=\">{xJ-e7j`t\"`Mq\"EG4:z9\"cggGo~0bJ|gcn5x,gF(4VusnXs]h62wQ\"lR=X?8Ae8ZP?WTsD4!u-g$MoeryeC9c<\\bi!O7rl`9-)GysHJ;t-+{^?Xf9PuW{~}aR{[8z*zg2R/t\"q6pHRBh;xA,9h2lU5WZS?l3HbyV0T\\X+^K>WJm>-`-1:J8Dh:LbvD!(4:pt~J"; 

fn main() {
    let plugboard = Plugboard::new(&[(b'B', b'U'),(b'`', b'N'),(b']', b'4'),(b'I', b'%'),(b'"', b'f'),(b'}', b'Z'),(b'D', b'+'),(b'A', b'9'),(b'3', b'8'),(b'*', b'2'),]
);
    let num_wires = 10;
    let ufw_b = Rotor::new(ORIG, UFW_B_MAP, 0);
    let r1 = Rotor::new(ORIG, R1_MAP, 12);
    let r2 = Rotor::new(ORIG, R2_MAP, 14);
    let r3 = Rotor::new(ORIG, R3_MAP, 47);

    let mut enigma = Enigma::new(ufw_b,r2,r1,r3, plugboard);

    // Reset enigma machine
    // Setup engima machine for attack
    enigma.rotor_settings(0, 0, 0);
    let plugboard = Plugboard::new(&[]);
    enigma.set_plugboard(plugboard);


    // ***************************
    // ***** ATTACK 
    // ***************************
    // Make 8 threads AND ATTACK


    let maximum_fitness = Arc::new(Mutex::new(0));
    let chosen_rotor_config = Arc::new(Mutex::new((0 as u8,0 as u8,0 as u8,0 as u64)));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let num_threads = 10u8;
    println!("\n-------------------------------Known Plaintext Attack------------------------------------");
    println!("Crib plaintext: {KNOWN_PLAINTEXT}\n");
    println!("Crib ciphertext: {KNOWN_CIPHERTEXT}\n");
    println!("\nBrute Force Attacking Rotors...");
    println!("\nStarting {num_threads} threads...");
    
    // Create threads
    for idx in 0..num_threads {
        let maximum_fitness = Arc::clone(&maximum_fitness);
        let chosen_rotor_config = Arc::clone(&chosen_rotor_config);

        let handle = thread::spawn(move || {
            let plugboard = Plugboard::new(&[]);
            let ufw_b = Rotor::new(ORIG, UFW_B_MAP, 0);
            let r1 = Rotor::new(ORIG, R1_MAP, 12);
            let r2 = Rotor::new(ORIG, R2_MAP, 14);
            let r3 = Rotor::new(ORIG, R3_MAP, 47);
            let mut enigma = Enigma::new(ufw_b,r2,r1,r3, plugboard);
            let partition_size: u8 = 94/num_threads + 1;
            let start: u8 = idx * partition_size;
            let end: u8 = std::cmp::min(start + partition_size - 1, 93);
            println!("Thread {idx} attacking: {start} 0 0 to {end} 93 93");
            for i in start..=end {
                for j in 0..94 {
                    for k in 0..94 {
                        enigma.rotor_settings(i,j,k);
                        let mut decrypted = String::new();
                        for c in KNOWN_CIPHERTEXT.chars() {
                            decrypted.push(enigma.encrypt(c as u8));
                        }
                        let f = fitness(&decrypted, &KNOWN_PLAINTEXT.to_string());
                        let mut max = maximum_fitness.lock().unwrap();
                            if f > *max {
                                println!("Rotors: {i} {j} {k}     \t Fitness: {f}");
                                *max = f;
                                let mut chosen_rotor = chosen_rotor_config.lock().unwrap();
                                *chosen_rotor = (i, j, k, f);
                            }
                        }
                    }
                }
                //println!("Thread {idx} completed");
            });
            handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    let chosen_rotor_config = *chosen_rotor_config.lock().unwrap();
    
    
    let (r, j, k, f) = chosen_rotor_config;
    println!("\nBest rotor configuration found: {r} {j} {k}");
    enigma.rotor_settings(r,j,k);
    let mut decrypted:String = String::new();
    for c in KNOWN_CIPHERTEXT.chars() {
        decrypted.push(enigma.encrypt(c as u8));
    }

    println!("\nDecrypted weather report before plugboard: \n{decrypted}\nFitness: {f}");

    println!("\nComparing with known plaintext to derive probable plugboard settings...");
    

    // ******************************
    // Find the plugboard settings
    // ******************************

    let mut current_fitness = f; // the current fitness of the text before plugboard with the correct rotor settings

    let mut chosen_wire_positions: Vec<(u8, u8)> = Vec::new(); 
    let mut wire_idx = 0u8;

    let (cr, cj, ck, _) = chosen_rotor_config;
    enigma.rotor_settings(cr, cj, ck);

    let mut printables: Vec<u8> = ORIG.to_vec();

    // Compare the cipher and plaintext based on the OPTIMUM rotor configuration
    for it in KNOWN_PLAINTEXT.chars().zip(decrypted.chars()) {
        if wire_idx >= num_wires {
            break;
        }
        let (a,b) = it;
        if a != b {
            // Try to connect cipher to known plaintext
            if printables.contains(&(a as u8)) && printables.contains(&(b as u8)) {
                // Push the wire position to be tested for fitness
                chosen_wire_positions.push((a as u8, b as u8));

                // Create a new enigma machine instance to test fitness of this plugboard setting
                let plugboardguessed = Plugboard::new(&(chosen_wire_positions.as_slice()));
                enigma.set_plugboard(plugboardguessed);  

                // Get the new decrypted with this setting
                let mut decrypted = "".to_string();
                for c in KNOWN_CIPHERTEXT.chars() {
                    decrypted.push(enigma.encrypt(c as u8));
                }
                enigma.rotor_settings(cr, cj, ck);

                let f = fitness(&decrypted, &KNOWN_PLAINTEXT.to_string());

                // Check the fitness of the text
                if f > current_fitness + 10 {
                    // println!("Current Fitness {} to {}", current_fitness, f);
                    // Possibly a correct wire connection
                    current_fitness = f;

                    // Drop the printables 
                    printables.retain(|x| *x != a as u8);
                    printables.retain(|x| *x != b as u8);
                    println!("Found: {} <-> {}", a, b);
                    
                    // Increment number of wires
                    wire_idx += 1;
                }
                else {
                    // probably not a correct wire connection
                    chosen_wire_positions.pop();
                }
            }
        }
        else{
            // same mapping, found a slot that is not mapped
            printables.retain(|x| *x != a as u8)
        }
    }

    println!("\nBrute forcing the rest of the plugboard wires...");

    // DO 1 wire at the time and find the best fitness
    // using position instead of mapping from cipher to plaintext
    while wire_idx < num_wires {
        let mut maximum = 0 as u8;
        let mut chosen_position = (0, 0);

        for s in 0..(printables.len()-1) { // plug first wire
            for t in s+1..printables.len() { // plug second wire
                // Create a new enigma machine instance to test fitness of this plugboard setting
                chosen_wire_positions.push((printables[s] as u8, printables[t] as u8));

                let plugboardguessed = Plugboard::new(chosen_wire_positions.as_slice());
                enigma.set_plugboard(plugboardguessed);            

                let mut decrypted = "".to_string();
                for c in KNOWN_CIPHERTEXT.chars() {
                    decrypted.push(enigma.encrypt(c as u8));
                }
                let f = fitness(&decrypted, &KNOWN_PLAINTEXT.to_string()) as u8;
                
                // if fitness f is better than use this configuration
                if f > maximum {
                    maximum = f;
                    chosen_position = (printables[s], printables[t]);
                    // println!("#{} wire connect between {} & {} with fitness {}", wire_idx, printables[s] as char, printables[t] as char, maximum)
                }

                // Remove the current wiring setting that we are trying
                chosen_wire_positions.pop();
                
                // Reset rotor settings
                enigma.rotor_settings(cr, cj, ck);
            }
        }

        // remove the available wire positions
        // Drop the printables 
        let (s, t) = chosen_position;
        
        printables.retain(|x| *x != s as u8);
        printables.retain(|x| *x != t as u8);
        println!("Found: {} <-> {}", s as char, t as char);

        chosen_wire_positions.push((s, t));

        // Iterate counter
        wire_idx += 1;
    }


    // use the chosen plugboard wire configuration and create new enigma machine
    let plugboard_chosen = Plugboard::new(&chosen_wire_positions);
    enigma.set_plugboard(plugboard_chosen);
    let (cr, cj, ck, _) = chosen_rotor_config;
    
    // Set rotor settings based on found best fit
    enigma.rotor_settings(cr, cj, ck);

    // Get final decrypted plaintext
    let mut decrypted = "".to_string();
    for c in KNOWN_CIPHERTEXT.chars() {
        decrypted.push(enigma.encrypt(c as u8));
    }
    
    // Print result
    println!("\nDecrypted plaintext from final configured: ");
    println!("{}", decrypted);
    assert_eq!(KNOWN_PLAINTEXT,decrypted);


    // DECRYPT flag with found enigma machine configuration
    enigma.rotor_settings(cr, cj, ck);
    let mut decrypted = "".to_string();
    for c in CIPHERTEXT_FLAG.chars() {
        decrypted.push(enigma.encrypt(c as u8));
    }

    println!("\nRotor Setting: {} {} {}", cr, cj, ck);
    println!("Plugboard Setting: {}", enigma.plugboard);
    println!("\nDecrypted Flag with final configuration: ");
    println!("{decrypted}");
    //println!("\n\nPress Enter to continue...");
    //let stdin = io::stdin();
    //let _ = stdin.read_line(&mut String::new());
}

fn fitness(s: &String, plaintext: &String) -> u64 {
    let mut counter = 0u64;
    for it in s.chars().zip(plaintext.chars()) {
        let (a,b) = it;
        if a == b {
            counter += 1; 
        }
    }
    counter
}
