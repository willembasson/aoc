use strict;

open(FH, "input.txt") or die "couldn't open";
my @v1 = ();
my @v2 = ();
while (<FH>) {
    chomp;
    my ($a, $b) = split /\s+/;
    push(@v1,$a);
    push(@v2,$b);
}
my $sum = 0;

my @v1s = sort(@v1);
my @v2s = sort(@v2);
for (my $i = 0; $i < scalar(@v1s); $i++) {
    $sum += abs($v1s[$i] - $v2s[$i]);
}
print $sum."\n";




