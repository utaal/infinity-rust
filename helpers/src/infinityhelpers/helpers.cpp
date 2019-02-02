#include "helpers.h"

namespace infinityhelpers {
  namespace core {
    infinity::core::Context * new_Context(uint16_t device, uint16_t devicePort) {
      return new infinity::core::Context(device, devicePort);
    }

    void delete_Context(infinity::core::Context * context) {
      delete context;
    }
  }

  namespace memory {
    void delete_RegionToken(infinity::memory::RegionToken * regionToken) {
      delete regionToken;
    }
  }
  
  namespace queues {
    void delete_QueuePair(infinity::queues::QueuePair * queuePair) {
      delete queuePair;
    }
  }
}
