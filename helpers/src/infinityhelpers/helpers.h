#include <infinity/queues/QueuePair.h>

namespace infinityhelpers {
  namespace core {
    infinity::core::Context * new_Context(uint16_t device, uint16_t devicePort);
    void delete_Context(infinity::core::Context * context);
  }
  namespace memory {
    void delete_RegionToken(infinity::memory::RegionToken * regionToken);
  }
  namespace queues {
    void delete_QueuePair(infinity::queues::QueuePair * queuePair);
  }
}
