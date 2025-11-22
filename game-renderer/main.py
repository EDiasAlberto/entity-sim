import pygame
import numpy as np
import state_processor as sp 


def render_terrain(game_state, colour_dict, terrain_width, terrain_height):
    
    # Get the map data as a tuple of numpy arrays (height_array, material_array)
    map_data = game_state.get_map_data()
    
    # map_data is a tuple of numpy arrays: (material_array, height_array)
    # We only need the material array for rendering colors
    material_array = map_data[0] if isinstance(map_data, tuple) else map_data
    
    # Reshape the material array to 2D if it's 1D
    if len(material_array.shape) == 1:
        material_array = material_array.reshape((terrain_height, terrain_width))
    
    # Create numpy array for terrain colors (RGB)
    terrain_array = np.zeros((terrain_height, terrain_width, 3), dtype=np.uint8)
    
    # Map each material ID to its corresponding color
    for material_id, color in colour_dict.items():
        mask = material_array == material_id
        terrain_array[mask] = color
    
    # Create surface and blit the terrain
    cellsize = 1
    width = terrain_array.shape[1] * cellsize
    height = terrain_array.shape[0] * cellsize
    
    surf = pygame.Surface((terrain_array.shape[1], terrain_array.shape[0]))
    # Note: pygame expects (width, height) ordering, numpy uses (height, width)
    pygame.surfarray.blit_array(surf, np.transpose(terrain_array, (1, 0, 2)))
    surf = pygame.transform.scale(surf, (width, height))
    
    return surf


def render_entities(surface, game_state, entity_color=(255, 255, 0), entity_size=3):
    """
    Draws entities on top of the terrain surface.
    
    Args:
        surface: pygame.Surface to draw entities on
        entity_mgmt: Dictionary containing entity management data with 'entities'
        entity_color: RGB color tuple for entities (default: yellow)
        entity_size: Radius of entity circles in pixels (default: 3)
    """
    entities = gs.get_entity_locations() 
    
    for entity_id, x, y in entities:
        # Draw entity as a circle
        pygame.draw.circle(surface, entity_color, (int(x), int(y)), entity_size)


# Initialize game state
gs = sp.generate_game_state((800, 800, 10), (200, 200, 400, 400), 3)

# Define material colors
colour_dict = {
    0: (255, 0, 0),   # Mud - Red
    1: (0, 255, 0),   # Grass - Green
    2: (0, 0, 255),   # Ice - Blue
}

# Initialize pygame
pygame.init()
(WIDTH, HEIGHT) = gs.get_terrain_map()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()

# Render terrain once
terrain_surface = render_terrain(gs, colour_dict, WIDTH, HEIGHT)

# Render entities on top of terrain
render_entities(terrain_surface, gs)

# Game loop
running = True
while running:
    clock.tick(60)
    
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
    
    screen.fill((0, 0, 0))
    screen.blit(terrain_surface, (0, 0))
    
    pygame.display.update()
            
pygame.quit()

