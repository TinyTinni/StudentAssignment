# Simple Student assignment

German only atm

## Das Problem

Wir haben $n$ viele Teilnehmer $s_i \in S$ für eine Vorlesung.
Diese Vorlesung beinhaltet zusätzliche Lehrstunden $t_k \in T$, wobei pro Lehrstunde nur eine begrenzte Anzahl $c_k$ an Teilnehmern teilnehmen kann.

Beispiel:

- 300 Teilnehmer: $s_1 .. s_{300} \in S$
- 10 Lehrstunden: $t_1..t_{30} \in T$
- max. 30 Teilnehmen pro Lehrstunde: $c_k = 30,  \forall k \in |T|$

Jetzt sind die Lehrstunden allerdings zu unterschiedlichen Zeiten. Die Teilnehmer können, je nach ihrer Präferenz, eine Wunschliste erstellen, zu welchen Lehrstunden sie teilnehmen möchten:

$$
w_{i,k} = \begin{cases} 1 &\text{, if }s_i\text{ wants to attend to }t_k\\0 &\text{, otherwise} \end{cases}
$$
  
Gesucht ist nun also eine Verteilung $A$:

$$
    a_{i,k} = \begin{cases} 1 &\text{, }s_i\text{ assigned to }t_k\\0 &\text{, otherwise} \end{cases}
$$
wobei auch gelten soll $a_{i,k} = 1 \implies w_{i,k} = 1$ und $\sum\limits_{i}a_{i,k} <= c_k$.

## ILP Solver

Diese Lösung nimmt an, dass alle Wünsche bedient werden können.
Hierbei können wir alle nicht gewünschten Lehrstunden schon a-prior auf 0 setzen:

$$
    w_{i,k} = 0 \implies a_{i,k} = 0
    \qquad, \forall i \in |S| ,k \in |T|
$$
Jeder Teilnehmer bekommt einen Platz in einer Lehrstunde:

$$
    \sum\limits_{k}^{|T|} a_{i,k} = 1
    \qquad, \forall i \in |S|
$$

Raumkapazität wird nicht überschritten:
$$
    \sum\limits_{i}^{|S|} a_{i,k} \le c_k
    \qquad, \forall k \in |T|
$$

## Optimizer

Der Solver konnte keine Lösung finden, es können also nicht alle Wünsche bedient werden.
Z3 hat einen Optimizer. Was ist nun eine geeignete Energy function?
Eine (unvollständige, bitte erweitern) Liste an Möglichkeiten:

- Minimum an verpassten timeslots

## Mögliche Erweiterungen (Notizen, unfertig)

- Spezialisierte Lehrstunden? Sodass man nicht nur nach Zeit sondern auch nach Spezialisierung sortieren kann z.B. Physiklehrer hat Zeit an $t_1$ und $t_3$, Schüler will Physik und hat Zeit and $t_3$ und $t_2$, will aber auch noch deutsch. Deutschlehrer hat Zeit an $t_1$ und $t_2$.

Spezialisierung $z_i$.

Beide müssen zum Zeitpunkt $t$ Zeit haben.

Schüler wünscht $z_i$:

$$
    z_i*a_{s,i} \le 1 \forall i
$$
Lehrer bietet $z$ an:

$$
    z_i*a_{t,i} \le 1 \forall i
$$
wobei $z=1$, wenn der Schüler die Thematik $z_i$ wünscht/Lehrer anbietet.

Lehrer kann Kurs $z$ anbieten mit Kapazität $c$:

$$
    \sum a_{s,i} \le c * (a_{t,i} \oplus a_{t+1,i}...)
$$

Lehrer Kapazität maximal $t_c$ pro Woche:

$$
    \sum\limits a_{t_i} \le t_c
$$

Alle Wünsche werden erfüllt:
