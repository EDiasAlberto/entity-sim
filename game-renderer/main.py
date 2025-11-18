import pygame
import numpy as np
import state_processor as sp 

terrain_map = sp.validate_and_run_terrain_gen(100, 100, 10)
colour_dict = {
    0: (255, 0, 0),
    1: (255, 0, 0),
    2: (0, 255, 0),
    3: (0, 0, 255) 
}

map = np.ndarray((100, 100, 3))

for i in range(100):
    for j in range(100):
        curr_idx = (i * 100) + j
        curr_cell = terrain_map['map'][curr_idx]
        map[i][j] = colour_dict[curr_cell['material']]

cellsize = 10
WIDTH = map.shape[0] * cellsize
HEIGHT = map.shape[1] * cellsize

pygame.init()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()
surf = pygame.Surface((map.shape[0], map.shape[1]))
pygame.surfarray.blit_array(surf, map)
surf = pygame.transform.scale(surf, (WIDTH, HEIGHT))

# game loop
running = True
while running:
    clock.tick(60)
    
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
            
    screen.fill((0, 0, 0))           
    screen.blit(surf, (0, 0))
    
    pygame.display.update()
            
pygame.quit()
