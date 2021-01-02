use std::collections::HashMap;
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn print2sum(inp_vec : &Vec<u32>)
{
    let mut expense_map : HashMap<u32, usize> = HashMap::new();
    for (i, x) in inp_vec.iter().enumerate() {
        expense_map.insert(*x , i);
        if expense_map.contains_key(&(2020 - *x))
        {
            println!("{}", *x * (2020 - *x));
        }
    }
}

fn print3sum(inp_vec : &Vec<u32>)
{
    let mut expense_map : HashMap<u32, (usize, usize)> = HashMap::new();
    for i in 0..inp_vec.len(){
        for j in i..inp_vec.len(){
            expense_map.insert(inp_vec[i] + inp_vec[j], (i , j));
        }
    }
    for (i, x) in inp_vec.iter().enumerate()
    {
        if expense_map.contains_key(&(2020 - x)){
            let (y , z) = expense_map[&(2020 - x)];
            println!("{}", x * inp_vec[y] * inp_vec[z]);
        }

    }
}

fn main () {
let raw_inp = r"1895
 1732
 1660
 1658
 1878
 367
 2010
 1989
 431
 1946
 1614
 2003
 945
 1856
 1934
 1937
 1781
 1947
 1991
 1917
 1604
 1707
 1966
 1959
 1182
 1828
 1880
 1908
 1942
 1687
 1611
 1922
 1913
 1803
 1976
 1718
 1885
 1971
 2000
 1912
 1981
 1776
 1901
 1941
 1935
 1977
 1907
 1893
 1898
 1975
 2001
 1833
 1951
 1939
 1988
 1870
 1985
 1932
 1930
 1938
 1926
 1931
 1982
 76
 1979
 657
 1872
 1933
 1961
 1987
 1998
 1994
 418
 1914
 1929
 1810
 2009
 1712
 830
 1990
 1900
 1876
 1753
 1859
 1965
 1963
 1905
 1921
 1685
 1694
 697
 1899
 1997
 1964
 1927
 1952
 1894
 1960
 1986
 1883
 1616
 1993
 1892
 1943
 2005
 1995
 1915
 1663
 1954
 1902
 1191
 1948
 1875
 1850
 1955
 1962
 1984
 1957
 1969
 1887
 1953
 1786
 1638
 1909
 1881
 603
 1973
 1784
 1869
 1925
 1968
 1737
 1807
 1950
 1992
 1936
 1918
 1891
 1897
 1940
 1919
 1910
 1862
 1958
 1832
 1904
 1791
 1920
 1874
 1729
 1643
 2007
 1871
 1999
 1584
 1890
 1924
 1974
 1701
 1906
 143
 1725
 1945
 1783
 1873
 1903
 167
 1855
 1633
 1956
 1996
 1808
 1884
 1916
 829
 2002
 1852
 1835
 1889
 1983
 1949
 1970
 1774
 1764
 1609
 1882
 1857
 2004
 1911
 1896
 1980
 2006
 1967
 2008
 1972
 1648
 1923
 1978
 1675
 1831";

//    let _sample_inp = [1721, 979, 366, 299, 675, 1456];

let inp_vec : Vec<u32> = raw_inp.split('\n').map(|s| remove_whitespace(&s).parse::<u32>().unwrap()).collect();
print2sum(&inp_vec);
print3sum(&inp_vec);
}
