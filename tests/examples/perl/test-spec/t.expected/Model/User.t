use strict;
use warnings;

use Test::Spec;
use Model::User;

describe "Model::User" => sub {
  it "works" => sub {
    is(1 + 1, 2);
  };
};

runtests;
