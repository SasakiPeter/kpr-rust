from collections import defaultdict
n, m, k = map(int, input().split())

MOD = 998_244_353
# n = 1000
# m = 5000
# DP

# conb = 0
# for i in range(1, m+1):
#     conb += 1+min(k-1, m-i)+min(k-1, i-1)

pair = []
dd = defaultdict(list)
for i in range(m):
    for j in range(m):
        if k <= abs(i-j):
            pair.append((i, j))
            dd[i].append(j)

print(len(pair))
print(len(dd.keys()))

dp = [[0]*m for _ in range(n)]
for i in range(m):
    dp[0][i] = 1

for i in range(n-1):
    for (j, k) in pair:
        dp[i+1][k] += dp[i][j]
        dp[i+1][k] %= MOD


# for row in dp:
#     print(row)

ans = sum(x for x in dp[-1])
ans %= MOD
print(ans)
