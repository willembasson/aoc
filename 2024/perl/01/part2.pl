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

my %hm = ();
for (my $i = 0; $i < scalar(@v2); $i++) {
    $hm{$v2[$i]}++;
}
for (my $i = 0; $i < scalar(@v1); $i++) {
    $sum += $v1[$i] * $hm{$v1[$i]};
}
print $sum."\n";



