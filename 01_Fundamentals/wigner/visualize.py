import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm
import scienceplots

# 조화 진동자의 매개변수
m = 1.0        # 질량
omega = 1.0    # 각진동수

# x, p 공간 정의
x = np.linspace(-3, 3, 300)
p = np.linspace(-3, 3, 300)
X, P = np.meshgrid(x, p)

# hbar 값들 정의
hbars = [1.0, 0.5, 0.1, 0.01]  # hbar 값이 점차 작아짐

with plt.style.context(["science", "nature"]):
    # 그래프 생성
    fig, axs = plt.subplots(2, 2, figsize=(10, 8), dpi=300, subplot_kw={"projection": "3d"})
    axs = axs.ravel()
    
    for i, hbar in enumerate(hbars):
        # 위그너 함수 계산
        W = (1 / (np.pi * hbar)) * np.exp(-(m * omega * X**2 + P**2 / (m * omega)) / hbar)
        
        # 3D 표면 그래프 그리기
        surf = axs[i].plot_surface(X, P, W, cmap=cm.viridis, linewidth=0, antialiased=False)
        axs[i].set_title(f"hbar = {hbar}")
        axs[i].set_xlabel('x')
        axs[i].set_ylabel('p')
        axs[i].set_zlabel('W(x, p)')
        
        # 컬러바 추가
        fig.colorbar(surf, ax=axs[i], shrink=0.5, aspect=10)
    
    plt.tight_layout()
    plt.savefig("wigner.png", dpi=300)
    
