from collections import defaultdict
import heapq


class LikeMultiSet:
    def __init__(self):
        self.hmin = []
        self.hmax = []
        self.d = defaultdict(int)

    def insert(self, x):
        heapq.heappush(self.hmin, x)
        heapq.heappush(self.hmax, -x)
        self.d[x] += 1

    def erase(self, x, c):
        if not self.d[x]:
            return 0
        else:
            self.d[x] = max(0, self.d[x]-c)
        while self.hmin:
            if not self.d[self.hmin[0]]:
                heapq.heappop(self.hmin)
                continue
            break
        while self.hmax:
            if not self.d[-self.hmax[0]]:
                heapq.heappop(self.hmax)
                continue
            break

    def get_min(self):
        if not self.hmin:
            return None
        else:
            return -self.hmax[0]-self.hmin[0]


q = int(input())
S = LikeMultiSet()
flag = False
for _ in range(q):
    n, *x = map(int, input().split())
    if n == 1:
        S.insert(x[0])
    elif n == 2:
        S.erase(*x)
    else:
        print(S.get_min())
        flag = True
if not flag:
    print()
