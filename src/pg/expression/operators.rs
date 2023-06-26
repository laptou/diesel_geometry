use diesel::pg::Pg;

infix_operator!(SameAs, " ~= ", backend: Pg);
infix_operator!(IsContainedBy, " <@ ", backend: Pg);
prefix_operator!(Length, " @-@ ", backend: Pg);
prefix_operator!(Center, " @@ ", backend: Pg);
prefix_operator!(PointCount, " # ", backend: Pg);
infix_operator!(Intersection, " # ", backend: Pg);
infix_operator!(ClosestPoint, " ## ", backend: Pg);
infix_operator!(Distance, " <-> ", backend: Pg);
infix_operator!(Contains, " @> ", backend: Pg);
infix_operator!(Overlaps, " && ", backend: Pg);
